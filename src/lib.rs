#![allow(unsafe_op_in_unsafe_fn)]
pub mod plugin_api;
pub mod data;
pub mod ui;
pub mod db;
pub mod il2cpp_types;
pub mod hooks;

use plugin_api::{InitResult, VtableV2};
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};

pub static HACHIMI_VERSION: AtomicI32 = AtomicI32::new(0);
pub static VTABLE_PTR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

pub unsafe fn get_real_target_addr(mut ptr: *mut u8) -> *mut c_void {
    if ptr.is_null() { return std::ptr::null_mut(); }

    for _ in 0..10 {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if *ptr == 0xE9 {
                let offset = std::ptr::read_unaligned(ptr.add(1) as *const i32);
                ptr = (ptr as usize).wrapping_add(5).wrapping_add(offset as isize as usize) as *mut u8;
                continue;
            } else if *ptr == 0xFF && *ptr.add(1) == 0x25 {
                let offset = std::ptr::read_unaligned(ptr.add(2) as *const i32);
                let target_ptr_addr = (ptr as usize).wrapping_add(6).wrapping_add(offset as isize as usize);
                ptr = std::ptr::read_unaligned(target_ptr_addr as *const usize) as *mut u8;
                continue;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            let instr = std::ptr::read_unaligned(ptr as *const u32);

            if (instr & 0xFC000000) == 0x14000000 {
                let mut offset = instr & 0x03FFFFFF;
                if offset & 0x02000000 != 0 {
                    offset |= 0xFC000000;
                }
                let byte_offset = (offset as i32) * 4;
                ptr = (ptr as usize).wrapping_add(byte_offset as isize as usize) as *mut u8;
                continue;
            }

            if (instr & 0xFF000000) == 0x58000000 {
                let rt = instr & 0x1F;
                let mut imm19 = (instr >> 5) & 0x0007FFFF;
                if imm19 & 0x00040000 != 0 {
                    imm19 |= 0xFFF80000;
                }
                let byte_offset = (imm19 as i32) * 4;

                let target_addr = std::ptr::read_unaligned(ptr.wrapping_add(byte_offset as usize) as *const usize);

                let next_instr = std::ptr::read_unaligned(ptr.add(4) as *const u32);
                if (next_instr & 0xFFFFFC1F) == 0xD61F0000 && ((next_instr >> 5) & 0x1F) == rt {
                    ptr = target_addr as *mut u8;
                    continue;
                }
            }

            if (instr & 0xFF800000) == 0xD2800000 {
                let rd = instr & 0x1F;
                let mut addr = ((instr >> 5) & 0xFFFF) as usize;
                let mut valid_chain = true;

                for (i, expected_base) in [(1, 0xF2A00000), (2, 0xF2C00000), (3, 0xF2E00000)].iter() {
                    let next_instr = std::ptr::read_unaligned(ptr.add(i * 4) as *const u32);
                    if (next_instr & 0xFF80001F) == (expected_base | rd) {
                        let imm16 = ((next_instr >> 5) & 0xFFFF) as usize;
                        addr |= imm16 << (i * 16);
                    } else {
                        valid_chain = false;
                        break;
                    }
                }

                if valid_chain {
                    let br_instr = std::ptr::read_unaligned(ptr.add(16) as *const u32);
                    if (br_instr & 0xFFFFFC1F) == 0xD61F0000 && ((br_instr >> 5) & 0x1F) == rd {
                        ptr = addr as *mut u8;
                        continue;
                    }
                }
            }

            if (instr & 0x9F000000) == 0x90000000 {
                let rd = instr & 0x1F;
                let immlo = (instr >> 29) & 3;
                let immhi = (instr >> 5) & 0x7FFFF;
                let mut imm21 = (immhi << 2) | immlo;
                if imm21 & 0x100000 != 0 {
                    imm21 |= 0xFFE00000;
                }
                let offset = (imm21 as i32 as isize) << 12;
                let page_addr = (ptr as usize) & !0xFFF;
                let target_page = page_addr.wrapping_add(offset as usize);

                let next_instr = std::ptr::read_unaligned(ptr.add(4) as *const u32);
                if (next_instr & 0xFF000000) == 0x91000000 {
                    let rn = (next_instr >> 5) & 0x1F;
                    let rd_add = next_instr & 0x1F;
                    let shift = (next_instr >> 22) & 3;
                    let mut imm12 = (next_instr >> 10) & 0xFFF;
                    if shift == 1 {
                        imm12 <<= 12;
                    }
                    if rn == rd {
                        let target_addr = target_page.wrapping_add(imm12 as usize);

                        let next2_instr = std::ptr::read_unaligned(ptr.add(8) as *const u32);
                        if (next2_instr & 0xFFFFFC1F) == 0xD61F0000 {
                            let rn2 = (next2_instr >> 5) & 0x1F;
                            if rn2 == rd_add {
                                ptr = target_addr as *mut u8;
                                continue;
                            }
                        }
                    }
                } else if (next_instr & 0xFFC00000) == 0xF9400000 {
                    let rn = (next_instr >> 5) & 0x1F;
                    let rd_ldr = next_instr & 0x1F;
                    let imm12 = ((next_instr >> 10) & 0xFFF) * 8;
                    if rn == rd {
                        let ptr_addr = target_page.wrapping_add(imm12 as usize);
                        let target_addr = std::ptr::read_unaligned(ptr_addr as *const usize);

                        let next2_instr = std::ptr::read_unaligned(ptr.add(8) as *const u32);
                        if (next2_instr & 0xFFFFFC1F) == 0xD61F0000 {
                            let rn2 = (next2_instr >> 5) & 0x1F;
                            if rn2 == rd_ldr {
                                ptr = target_addr as *mut u8;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        break;
    }

    ptr as *mut c_void
}

fn internal_init(_version: i32) -> InitResult {
    unsafe {
        let vtable_ptr = VTABLE_PTR.load(Ordering::SeqCst);
        let vtable = &*(vtable_ptr as *const VtableV2);
        (vtable.gui_register_menu_section)(Some(ui::render_optimizer_ui), std::ptr::null_mut());

        let _ = db::load_skill_database();

        let _ = &*data::SKILL_SCORES;

        let image = (vtable.il2cpp_get_assembly_image)(c"umamusume".as_ptr());
        let interceptor = (vtable.hachimi_get_interceptor)((vtable.hachimi_instance)());

        hooks::install(vtable, interceptor as *mut c_void, image as *mut c_void);
    }

    InitResult::Ok
}

#[unsafe(no_mangle)]
pub extern "C" fn hachimi_init(vtable_ptr: *const c_void, version: i32) -> InitResult {
    HACHIMI_VERSION.store(version, Ordering::SeqCst);
    VTABLE_PTR.store(vtable_ptr as *mut c_void, Ordering::SeqCst);

    internal_init(version)
}

#[unsafe(no_mangle)]
pub extern "C" fn hachimi_init_v3(get_api: plugin_api::HachimiGetApiFn, version: i32) -> InitResult {
    HACHIMI_VERSION.store(version, Ordering::SeqCst);

    let vtable_ptr = unsafe {
        let dynamic_vtable = Box::new(plugin_api::VtableV3::from_get_api(get_api));
        Box::leak(dynamic_vtable) as *const plugin_api::VtableV3 as *mut c_void
    };

    VTABLE_PTR.store(vtable_ptr, Ordering::SeqCst);

    internal_init(version)
}