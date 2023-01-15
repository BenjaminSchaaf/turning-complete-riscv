#![no_std]
#![no_main]

// use arrayvec::ArrayString;

mod init;
mod peripherals;
mod keyboard;
mod console;
mod utils;
mod examples;

#[export_name = "main"]
fn main() -> ! {
    let mut io = peripherals::Peripherals::take().unwrap();

    // examples::image::run(&mut io);
    examples::chess::run(io);

    // let mut console = console::Console::new(io.con);
    // let mut keyboard = keyboard::Keyboard::new(io.kbd);

    // let mut buf = ArrayString::<80>::new();
    // loop {
    //     utils::input_line(&mut console, &mut keyboard, &mut buf);
    //     buf.clear();
    // }

    loop {}
}
