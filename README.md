# Penelope OS
Now written in Rust!
---
Penelope OS is a simple operating system written in Rust.
I decided to make this out of boredom and procrastination from my university exam studies.

I'm not sure how far I'll take this project, but I'll try to make it as good as I can.

This project is inspired by the [Writing an OS in Rust](https://os.phil-opp.com/) blog series by Philipp Oppermann. So much of the original code is based on his work.
However, I'm trying to make this my own project by adding my own features and code to it.

Also, this is my second Penelope OS project, the first one was written in C and was also made out of procrastination from last year's university exam studies.

## Building
To build the project, you need to have the following tools installed:
- Rust
- Cargo
- QEMU (If you want to run the OS in a virtual machine using the `cargo run` command and don't want to modify the `.cargo/config.toml` file to use another emulator)

{Build instructions will be added later}

## Features
- [x] Booting
- [x] VGA Buffer Driver
- [x] Serial Driver
- [x] CPU Interrupts (Want to rewrite to use without the `x86_64` crate)
- [x] Paging
- [x] Heap Allocation (Bump, Fixed Size, Linked List)
- [x] Keyboard Driver
- [x] Shell
- [ ] File System
- [ ] Async/Await