#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate rlibc;
use core::arch::asm;
use core::panic::PanicInfo;

use kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

fn background_render(w: usize, h: usize, graphics: &mut Graphics) {
    for y in 0..(h) {
        for x in 0..(w) {
            unsafe { graphics.write_pixel(x, y, PixelColor(10, 10, 10)) };
        }
    }
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };
    let mut console = Graphics::new(fb, mi);
    let (width, height) = (&console).resolution();
    let _div = 8;

    background_render(width, height, &mut console);

    let word: &str = "Hello, Musix!\n$ ";

    console.print(100, 100, word);

    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
