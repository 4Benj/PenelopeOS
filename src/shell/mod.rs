use crate::{drivers::keyboard::read_key, hlt_loop, print, println, serial_println};
use pc_keyboard::DecodedKey;
use alloc::string::String;
use alloc::vec::Vec;

pub fn shell() {
    serial_println!("[Shell] Initializing");
    loop {
        print!("> ");
        let command = read_line();
        let mut args = command.split_whitespace();
        if let Some(cmd) = args.next() {
            match cmd {
                "echo" => {
                    let output: String = args.collect::<Vec<&str>>().join(" ");
                    println!("{}", output);
                }
                "exit" => {
                    println!("Exiting shell.");
                    break;
                }
                "panik" => {
                    // Trigger a panic
                    panic!("You triggered a panic!");
                }
                _ => {
                    println!("Unknown command: {}", cmd);
                }
            }
        }
    }
    serial_println!("[Shell] Ending");
}

fn read_line() -> String  {
    serial_println!("[Shell] Reading Line");
    let mut input = String::new();
    loop {
        if let Some(key) = read_key() {
            match key {
                DecodedKey::Unicode(c) => match c {
                    '\n' => {
                        println!();
                        break;
                    }
                    '\u{8}' => {
                        if input.len() > 0 {
                            print!("\u{8} \u{8}");
                            input.pop();
                        }
                        continue;
                    }
                    _ => {
                        input.push(c);
                        print!("{}", c);
                    }
                },
                _ => {}
            }
        } else {
            x86_64::instructions::hlt();
        }
    }
    input
}