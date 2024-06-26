#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)] // enable custom test frameworks
#![test_runner(penelope::test_runner)] // set the test runner
#![reexport_test_harness_main = "test_main"] // rename the test harness function to test_main

const VERSION: &str = env!("CARGO_PKG_VERSION");

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use penelope::{drivers::vga_buffer::Color, println, shell::shell};

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

/**
 * The entry point of the OS's kernel.
 * All Rust code must be called from this function.
 */
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    use x86_64::VirtAddr;
    use penelope::arch::x86_64::memory::{self, BootInfoFrameAllocator};
    use penelope::arch::x86_64::allocator;


    println!("PENELOPE OS {}", VERSION);
    penelope::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    shell();

    println!("It did not crash!");
    penelope::hlt_loop();
}

/**
 * This function is called on panic.
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use penelope::drivers::vga_buffer::WRITER;
    WRITER.lock().set_color(Color::Red, Color::Black);
    println!("{}", _info);
    WRITER.lock().set_color(Color::White, Color::Black);
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