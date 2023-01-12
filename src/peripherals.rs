use core::ops::{Deref, DerefMut};
use core::arch::asm;

#[allow(dead_code)]
pub fn ecall() {
    unsafe { asm!("ecall"); }
}

#[allow(dead_code)]
pub fn ebreak() {
    unsafe { asm!("ebreak"); }
}

pub type Console = [u8; 80 * 24];

pub struct CON {}
unsafe impl Send for CON {}
impl CON {
    pub const fn ptr() -> *mut Console {
        0x400 as *mut _
    }
}
impl Deref for CON {
    type Target = Console;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CON::ptr() }
    }
}
impl DerefMut for CON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *CON::ptr() }
    }
}

pub static mut DEVICE_PERIPHERALS: bool = false;

pub struct Peripherals {
    pub con: CON,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) }
    }

    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;

        Peripherals {
            con: CON {}
        }
    }
}
