#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate rlibc;
use core::arch::asm;
use core::panic::PanicInfo;

use kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };
    let mut graphics = Graphics::new(fb, mi);
    let (width, height) = graphics.resolution();

    unsafe {
        for y in 0..(height) {
            for x in 0..(width) {
                graphics.write_pixel(x, y, PixelColor(10, 10, 10));
            }
        }
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
