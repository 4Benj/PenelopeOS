use crate::{drivers::keyboard::read_key, hlt_loop, print, println, serial_println};
use pc_keyboard::DecodedKey;

const MAX_INPUT_SIZE: usize = 256;

pub fn shell() {
    serial_println!("Starting shell");
    loop {
        print!("> ");
        let mut buffer = [0u8; MAX_INPUT_SIZE];
        let input_length = read_line(&mut buffer);
        if input_length == 0 {
            continue;
        }
        let command = core::str::from_utf8(&buffer[..input_length]).unwrap_or("");
        execute_command(command);
    }
    serial_println!("Shell exited");
}

fn read_line(buffer: &mut [u8]) -> usize {
    let mut index = 0;
    loop {
        if let Some(key) = read_key() {
            serial_println!("Key: {:?}", key);
            match key {
                DecodedKey::Unicode(character) => match character {
                    '\n' => {
                        println!();
                        break;
                    }
                    '\u{8}' => {
                        if index > 0 {
                            index -= 1;
                            print!("\u{8} \u{8}");
                        }
                        continue;
                    }
                    _ => {
                        if index < buffer.len() {
                            buffer[index] = character as u8;
                            index += 1;
                            print!("{}", character);
                        }
                    }
                },
                _ => {}
            }
        } else {
            x86_64::instructions::hlt();
        }
    }
    index
}

fn execute_command(command: &str) {
    serial_println!("Executing command: {}", command);

    let mut parts = command.split_whitespace();
    if let Some(cmd) = parts.next() {
        match cmd {
            "echo" => {
                for arg in parts {
                    print!("{} ", arg);
                }
                println!();
            }
            "exit" => {
                println!("Exiting shell.");
                // Handle exit logic, if needed
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
