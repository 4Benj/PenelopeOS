use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::{Port, PortGeneric};

use crate::serial_println;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
        serial_println!("[Keyboard] Initializing Keyboard with Us104Key and ScancodeSet1");
        Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore))
    };
}

const KEY_BUFFER_SIZE: usize = 256;
lazy_static! {
    static ref KEY_BUFFER: Mutex<[Option<DecodedKey>; KEY_BUFFER_SIZE]> = Mutex::new([None; KEY_BUFFER_SIZE]);
    static ref BUFFER_START: Mutex<usize> = Mutex::new(0);
    static ref BUFFER_END: Mutex<usize> = Mutex::new(0);
}

pub fn keyboard_interrupt() {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    if let Ok(Some(key_event)) = keyboard.add_byte(unsafe { port.read() }) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let mut buffer = KEY_BUFFER.lock();
            let mut end = BUFFER_END.lock();
            buffer[*end] = Some(key);
            *end = (*end + 1) % KEY_BUFFER_SIZE;
            match key {
                DecodedKey::Unicode(character) => {
                    serial_println!("[Keyboard] Unicode: {}", character);
                }
                DecodedKey::RawKey(key) => {
                    serial_println!("[Keyboard] RawKey: {:?}", key);
                }
                _ => {}
            }
        }
    }

}

pub fn read_key() -> Option<DecodedKey> {
    let mut buffer = KEY_BUFFER.lock();
    let mut start = BUFFER_START.lock();
    let mut end = BUFFER_END.lock();

    if *start == *end {
        None
    } else {
        let key = buffer[*start].take();
        *start = (*start + 1) % KEY_BUFFER_SIZE;
        key
    }
}