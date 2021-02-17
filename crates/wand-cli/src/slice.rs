use wasmtime::Memory;

use crate::traits::LoHi;

pub struct WasmSlice {
    ptr: u32,
    len: u32,
}

impl WasmSlice {
    fn from_u64(ptr: u64) -> WasmSlice {
        WasmSlice {
            ptr: ptr.lo(),
            len: ptr.hi(),
        }
    }

    pub unsafe fn get_buf<'memory>(memory: &'memory Memory, ptr: u64) -> &'memory [u8] {
        let slice = WasmSlice::from_u64(ptr);
        slice.as_buf(memory)
    }

    pub unsafe fn get_str<'memory>(memory: &'memory Memory, ptr: u64) -> &'memory str {
        let bytes = WasmSlice::get_buf(memory, ptr);
        std::str::from_utf8_unchecked(bytes)
    }

    pub unsafe fn as_buf<'memory>(&self, memory: &'memory Memory) -> &'memory [u8] {
        &memory.data_unchecked()[(self.ptr as usize)..][..(self.len as usize)]
    }
}
