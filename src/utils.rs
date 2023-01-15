use arrayvec::ArrayString;
use crate::console::Console;
use crate::keyboard::{Keyboard, Key};
use core::fmt::Write;

pub fn input_line<const CAP: usize>(console: &mut Console, keyboard: &mut Keyboard, result: &mut ArrayString<CAP>) {
    loop {
        if let Some((input, chr)) = keyboard.pop_input() {
            if let Some(chr) = chr {
                if chr == '\n' {
                    console.next_line();
                    return;
                }

                write!(console, "{}", chr).unwrap();
                result.push(chr);
            } else if !input.keyup() {
                match Key::from_keyval(input.keyval()) {
                    Some(Key::Backspace) => {
                        if result.len() > 0 {
                            console.erase();
                            result.pop();
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}
