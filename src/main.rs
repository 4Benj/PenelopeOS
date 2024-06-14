#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    use core::fmt::Write;
    println!("Hello World{}", "!");
    println!("This is a test of the VGA buffer driver{}", "!");
    print!("on Penelope OS v0.1.0");
    print!(" written in Rust!");

    // we do nothing here
    loop {}
}