use std::ffi::c_void;

#[repr(C)]
pub struct Il2CppObject {
    pub klass: *mut c_void,
    pub monitor: *mut c_void,
}

#[repr(C)]
pub struct Il2CppString {
    pub object: Il2CppObject,
    pub length: i32,
    pub chars: [u16; 1],
}

impl Il2CppString {
    pub fn as_string(&self) -> String {
        let slice = unsafe { std::slice::from_raw_parts(self.chars.as_ptr(), self.length as usize) };
        String::from_utf16_lossy(slice)
    }
}

#[repr(C)]
pub struct Il2CppArray {
    pub obj: Il2CppObject,
    pub bounds: *mut c_void,
    pub max_length: usize,
}

impl Il2CppArray {
    pub unsafe fn get_obj(&self, index: usize) -> *mut Il2CppObject {
        let data_ptr = (self as *const _ as *const u8).add(32) as *const *mut Il2CppObject;
        *data_ptr.add(index)
    }

    pub unsafe fn set_obj(&mut self, index: usize, val: *mut Il2CppObject) {
        let data_ptr = (self as *mut _ as *mut u8).add(32) as *mut *mut Il2CppObject;
        *data_ptr.add(index) = val;
    }

    pub unsafe fn get_i32_mut_slice(&mut self) -> &mut [i32] {
        let data_ptr = (self as *mut _ as *mut u8).add(32) as *mut i32;
        std::slice::from_raw_parts_mut(data_ptr, self.max_length)
    }
}