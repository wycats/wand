use wasmtime::Memory;

use crate::traits::LoHi;

#[derive(Debug)]
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

    pub fn get_buf<'memory>(memory: &'memory Memory, ptr: u64) -> anyhow::Result<Vec<u8>> {
        let slice = WasmSlice::from_u64(ptr);
        slice.as_buf(memory)
    }

    pub fn get_str<'memory>(memory: &'memory Memory, ptr: u64) -> anyhow::Result<String> {
        let bytes = WasmSlice::get_buf(memory, ptr)?;
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    }

    pub fn as_buf<'memory>(&self, memory: &'memory Memory) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.len as usize);
        unsafe { buf.set_len(self.len as usize) };
        memory.read(self.ptr as usize, &mut buf)?;
        Ok(buf)
        // &memory.data_unchecked()[(self.ptr as usize)..][..(self.len as usize)]
    }
}
