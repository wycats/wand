use alloc::string::String;
use alloc::vec::Vec;
use core::{intrinsics::transmute, slice};

extern "C" {
    pub fn log_str(ptr: u64);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WasmString {
    ptr: u32,
    len: u32,
}

impl From<&String> for WasmString {
    fn from(string: &String) -> Self {
        WasmString {
            ptr: string.as_ptr() as u32,
            len: string.len() as u32,
        }
    }
}

impl From<&'static str> for WasmString {
    fn from(string: &'static str) -> Self {
        WasmString {
            ptr: string.as_ptr() as u32,
            len: string.len() as u32,
        }
    }
}

impl Into<String> for WasmString {
    fn into(self) -> String {
        let slice = unsafe { slice::from_raw_parts(self.ptr as *const u8, self.len as usize) };
        let vec = Vec::from(slice);
        unsafe { String::from_utf8_unchecked(vec) }
        // String::from_raw_parts(buf, length, capacity)
    }
}

impl WasmString {
    pub fn log(&self) {
        unsafe {
            log_str(transmute(*self));
        }
    }
}
