use core::{arch::global_asm, panic::PanicInfo};
use crate::peripherals;

global_asm!(include_str!("init.s"));
// global_asm!(include_str!("tests.s"));

// A basic panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    peripherals::ebreak();
    loop {}
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section
    static _sidata: u32;
}

#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    main();
}
