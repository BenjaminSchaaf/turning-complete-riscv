#![no_std]
#![no_main]

mod init;
mod peripherals;

#[export_name = "main"]
fn main() -> ! {
    unsafe { peripherals::DEVICE_PERIPHERALS = false };

    let mut io = peripherals::Peripherals::take().unwrap();

    // Print to the console
    for (i, c) in "Hello World!".as_bytes().iter().enumerate() {
        io.con[i] = *c;
    }

    loop {}
}
