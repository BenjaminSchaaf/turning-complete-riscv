use crate::peripherals::{KeyboardInput, KBD};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitfield::bitfield;

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    Tilde = 13,
    Num1 = 14,
    Num2 = 15,
    Num3 = 16,
    Num4 = 17,
    Num5 = 18,
    Num6 = 19,
    Num7 = 20,
    Num8 = 21,
    Num9 = 22,
    Num0 = 23,
    Minus = 24,
    Equals = 25,
    Backspace = 26,
    Tab = 27,
    Q = 28,
    W = 29,
    E = 30,
    R = 31,
    T = 32,
    Y = 33,
    U = 34,
    I = 35,
    O = 36,
    P = 37,
    OpenSquareBracket = 38,
    CloseSquareBracket = 39,
    Backslash = 40,
    CapsLock = 41,
    A = 42,
    S = 43,
    D = 44,
    F = 45,
    G = 46,
    H = 47,
    J = 48,
    K = 49,
    L = 50,
    Semicolon = 51,
    Quote = 52,
    Enter = 53,
    Shift = 54,
    Z = 55,
    X = 56,
    C = 57,
    V = 58,
    B = 59,
    N = 60,
    M = 61,
    Comma = 62,
    Dot = 63,
    Slash = 64,
    Ctrl = 65,
    Super = 66,
    Alt = 67,
    Space = 68,
    Left = 69,
    Right = 70,
    Up = 71,
    Down = 72,
}

const ASCII_KEYMAP_OFFSET: usize = 13;
const ASCII_KEYMAP: &[u8; 56] = b"`1234567890-=\0\0qwertyuiop[]\\\0asdfghjkl;'\n\0zxcvbnm,./\0\0\0 ";
const ASCII_KEYMAP_UPPER: &[u8; 56] = b"~!@#$%^&*()_+\0\0QWERTYUIOP{}|\0ASDFGHJKL:\"\0\0ZXCVBNM<>?\0\0\0 ";

impl Key {
    pub fn to_ascii(&self) -> Option<(char, char)> {
        let index = *self as u8;

        if index as usize >= ASCII_KEYMAP_OFFSET {
            let index = index as usize - ASCII_KEYMAP_OFFSET;
            if index < ASCII_KEYMAP.len() {
                let lower = ASCII_KEYMAP[index];
                if lower == 0 {
                    None
                } else {
                    Some((lower as char, ASCII_KEYMAP_UPPER[index] as char))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn from_keyval(item: u8) -> Option<Key> {
        FromPrimitive::from_u8(item)
    }
}

bitfield!{
    #[derive(Clone, Copy)]
    pub struct Modifiers(u8);
    u8;
    pub shift, set_shift: 0;
    // pub ctrl, set_ctrl: 1;
    // pub alt, set_alt: 2;
    // pub caps_lock, set_caps_lock: 3;
}

pub struct Keyboard {
    kbd: KBD,
    modifiers: Modifiers,
}

impl Keyboard {
    pub fn new(kbd: KBD) -> Keyboard {
        Keyboard {
            kbd,
            modifiers: Modifiers(0),
        }
    }

    pub fn pop_input(&mut self) -> Option<(KeyboardInput, Option<char>)> {
        if let Some(input) = self.kbd.pop_input() {
            let key = Key::from_keyval(input.keyval())?;

            match key {
                Key::Shift => self.modifiers.set_shift(!input.keyup()),
                _ => {},
            }

            if input.keyup() {
                return Some((input, None));
            }

            if let Some((lower, upper)) = key.to_ascii() {
                Some((input, Some(if self.modifiers.shift() { upper } else { lower })))
            } else {
                Some((input, None))
            }
        } else {
            None
        }
    }
}
