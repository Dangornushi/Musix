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
    let div = 8;

    background_render(width, height, &mut console);

    console.putchar(100, 100, 'H');
    console.putchar(100 + div, 100, 'e');
    console.putchar(100 + div * 2, 100, 'l');
    console.putchar(100 + div * 3, 100, 'l');
    console.putchar(100 + div * 4, 100, 'o');
    console.putchar(100 + div * 5, 100, ' ');
    console.putchar(100 + div * 6, 100, 'd');
    console.putchar(100 + div * 7, 100, 'a');
    console.putchar(100 + div * 8, 100, 'n');
    console.putchar(100 + div * 9, 100, 'g');
    console.putchar(100 + div * 10, 100, 'o');
    console.putchar(100 + div * 11, 100, 'm');
    console.putchar(100 + div * 12, 100, 'u');
    console.putchar(100 + div * 13, 100, 's');
    console.putchar(100 + div * 14, 100, 'h');
    console.putchar(100 + div * 15, 100, 'i');
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
