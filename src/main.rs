#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // enable custom test frameworks
#![test_runner(penelope::test_runner)] // set the test runner
#![reexport_test_harness_main = "test_main"] // rename the test harness function to test_main

const VERSION: &str = env!("CARGO_PKG_VERSION");

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

use penelope::{println};

entry_point!(kernel_main);

/**
 * The entry point of the OS's kernel.
 * All Rust code must be called from this function.
 */
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    use penelope::memory::BootInfoFrameAllocator;

    println!("PENELOPE OS {}", VERSION);
    penelope::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    penelope::hlt_loop();
}

/**
 * This function is called on panic.
 */
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    penelope::hlt_loop();
}

/**
 * This function is called on panic but only in test mode.
 */
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    penelope::test_panic_handler(info)
}