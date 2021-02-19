#![no_std]

// #![cfg_attr(not(std), no_std)]
// #![feature(alloc_error_handler)]

extern crate alloc;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use alloc::string::{String, ToString};
use panic::UnwrapWasm;
use string::WasmString;
use transpile::TranspileModule;

mod emitter;
mod error;
mod panic;
mod rewrite_ext;
mod string;
mod transpile;
mod writer;
use alloc::format;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub fn allocate(len: usize) -> *mut u8 {
    let mut buf = alloc::vec::Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub extern "C" fn hello(ptr: u32, len: u32) -> u64 {
    let vec = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };

    let str = unsafe { core::str::from_utf8_unchecked(vec) }
        .clone()
        .to_string();

    // WasmString::from(&str).log();
    let parse =
        TranspileModule::parse(str, "index.ts".into()).expect_wasm(|_| "parse failed".into());
    let string: String = parse.into();
    WasmString::transfer(string)
}
