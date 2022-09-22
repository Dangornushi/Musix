#![no_std]
#![no_main]

mod graphics;
use core::arch::asm;
use graphics::{FrameBuffer, ModeInfo};

// 2022/9/21
// Musix Kernel

#[derive(Copy, Clone, Debug)]
pub struct PixelColor(pub u8, pub u8, pub u8); // RGB

#[no_mangle]
pub extern "sysv64" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
