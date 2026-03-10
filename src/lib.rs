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
    for _ in 0..5 {
        if *ptr == 0xE9 {
            let offset = std::ptr::read_unaligned(ptr.add(1) as *const i32);
            ptr = (ptr as usize).wrapping_add(5).wrapping_add(offset as isize as usize) as *mut u8;
        } else if *ptr == 0xFF && *ptr.add(1) == 0x25 {
            let offset = std::ptr::read_unaligned(ptr.add(2) as *const i32);
            let target_ptr_addr = (ptr as usize).wrapping_add(6).wrapping_add(offset as isize as usize);
            ptr = std::ptr::read_unaligned(target_ptr_addr as *const usize) as *mut u8;
        } else {
            break;
        }
    }
    ptr as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn hachimi_init(vtable_ptr: *const c_void, version: i32) -> InitResult {
    HACHIMI_VERSION.store(version, Ordering::SeqCst);
    VTABLE_PTR.store(vtable_ptr as *mut c_void, Ordering::SeqCst);

    unsafe {
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