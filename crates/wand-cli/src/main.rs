//! Example of instantiating of instantiating a wasm module which uses WASI
//! imports.

// You can execute this example with `cargo run --example wasi`

pub mod slice;
pub mod traits;
pub mod wasm;

use wasmtime::Caller;

use crate::slice::WasmSlice;
use crate::traits::GetMemory;
use crate::wasm::WasmEngine;

const BYTES: &'static [u8] = include_bytes!("../data/wand.wasm");

fn main() -> anyhow::Result<()> {
    let wasm = WasmEngine::new()?;
    let mut linker = wasm.linker();

    linker.func("env", "log_str", |caller: Caller, ptr: u64| {
        println!("{}", unsafe { WasmSlice::get_str(&caller.memory(), ptr) });
    })?;

    let source = "function hello(): string { return 'hello' }".to_string();

    let module = linker.instantiate_module(BYTES)?;

    println!("instantiated");

    let mem = module.memory();
    let len = source.len() as u32;
    let ptr: u32 = module.call1("allocate", len as u32).unwrap();
    let slice = mem.data_ptr();

    let zero = unsafe { slice.offset(ptr as isize) };

    unsafe { zero.copy_from(source.as_ptr(), source.len()) };

    // slice[ptr] = len;
    module.call2("hello", ptr, source.len() as u32)?;

    Ok(())
}
