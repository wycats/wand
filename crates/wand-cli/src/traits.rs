use wasmtime::{Caller, Memory};

pub trait LoHi {
    type Output;

    fn lo(&self) -> Self::Output;
    fn hi(&self) -> Self::Output;
}

impl LoHi for u16 {
    type Output = u8;

    fn lo(&self) -> Self::Output {
        *self as u8
    }
    fn hi(&self) -> Self::Output {
        (*self >> 8) as u8
    }
}

impl LoHi for u32 {
    type Output = u16;

    fn lo(&self) -> Self::Output {
        *self as u16
    }
    fn hi(&self) -> Self::Output {
        (*self >> 16) as u16
    }
}

impl LoHi for u64 {
    type Output = u32;

    fn lo(&self) -> Self::Output {
        *self as u32
    }
    fn hi(&self) -> Self::Output {
        (*self >> 32) as u32
    }
}

pub trait GetMemory {
    fn memory(&self) -> Memory;
}

impl GetMemory for Caller<'_> {
    fn memory(&self) -> Memory {
        self.get_export("memory")
            .expect("expected a 'memory' export")
            .into_memory()
            .expect("expected 'memory' export to be memory")
    }
}
