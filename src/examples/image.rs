/*
Use image_to_argb32.py to export an image into res/image.rgb
*/
use crate::peripherals;

pub fn run(io: &mut peripherals::Peripherals) {
    let image = include_bytes!("../../res/image.rgb");
    let mut buf: &[u8] = image;

    for i in 0..96*64 {
        let bytes: [u8; 4] = buf[0..4].try_into().unwrap();
        io.dsp.write_pixel(i, u32::from_le_bytes(bytes));
        buf = &buf[4..];
    }
    peripherals::ebreak();
    io.dsp.flush();
}
