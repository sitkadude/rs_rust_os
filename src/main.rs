#![no_std] // Don't link the Rust standard library.
#![no_main] // Disables all Rust-level entry points.

use core::panic::PanicInfo;

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler] // This function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

