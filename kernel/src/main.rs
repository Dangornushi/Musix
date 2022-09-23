#![no_std]
#![no_main]

pub mod graphics;

use core::arch::asm;
use graphics::{FrameBuffer, Graphics, ModeInfo};
// 2022/9/21
// Musix Kernel

#[derive(Copy, Clone, Debug)]
pub struct PixelColor(pub u8, pub u8, pub u8); // RGB

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: *mut FrameBuffer, mode_info: *mut ModeInfo) {
    unsafe {
        let rgb = PixelColor {
            0: 255_u8,
            1: 255_u8,
            2: 255_u8,
        };
        /*
         * Err
                let mut Console = Graphics::initialize_instance(frame_buffer, mode_info);
                let mut Console = Graphics { Console };
                Console.write_actual_pixel(10 + 1, 10 + 1, rgb);
        */
        /*
                frame_buffer_config.frame_buffer.as_mut_ptr();

                for x in 0..frame_buffer_config.horizontal.0 as usize {
                    for y in 0..frame_buffer_config.horizontal.0 as usize {
                        frame_buffer_config
                            .frame_buffer
                            .write_value(y + x * 4, [rgb.0, rgb.1, rgb.2])
                    }
                }
        */
        loop {
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
