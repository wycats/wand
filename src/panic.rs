use crate::string::WasmString;

pub trait UnwrapWasm<T> {
    type Error;

    fn expect_wasm(self, cb: impl FnOnce(Self::Error) -> WasmString) -> T;
}

impl<T, E> UnwrapWasm<T> for Result<T, E> {
    type Error = E;

    fn expect_wasm(self, cb: impl FnOnce(Self::Error) -> WasmString) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let string = cb(err);
                string.log();
                panic!()
            }
        }
    }
}

impl<T> UnwrapWasm<T> for Option<T> {
    type Error = ();

    fn expect_wasm(self, cb: impl FnOnce(Self::Error) -> WasmString) -> T {
        match self {
            Some(val) => val,
            None => {
                let string = cb(());
                string.log();
                panic!()
            }
        }
    }
}

// #[cfg(not(std))]
// #[cfg(target_arch = "wasm32")]
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     // let string = "a panic occurred".to_string();
//     // let panic = WasmString::borrow(&string);
//     // panic.log();

//     WasmString::from("a panic occurred").log();

//     loop {}
// }

// #[cfg(not(std))]
// #[alloc_error_handler]
// fn my_example_handler(layout: core::alloc::Layout) -> ! {
//     panic!("memory allocation of {} bytes failed", layout.size())
// }
