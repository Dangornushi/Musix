#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate rlibc;
use core::arch::asm;
use core::panic::PanicInfo;

use kernel::console::Console;
use kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };
    let mut console = Console::new(fb, mi);

    let ascii: &str = " /$$      /$$                     /$$\n| $$$    /$$$                    |__/\n| $$$$  /$$$$ /$$   /$$  /$$$$$$$ /$$ /$$   /$$\n| $$ $$/$$ $$| $$  | $$ /$$_____/| $$|  $$ /$$/\n| $$  $$$| $$| $$  | $$|  $$$$$$ | $$ \\  $$$$/\n| $$\\  $ | $$| $$  | $$ \\____  $$| $$  >$$  $$\n| $$ \\/  | $$|  $$$$$$/ /$$$$$$$/| $$ /$$/\\  $$\n|__/     |__/ \\______/ |_______/ |__/|__/  \\__/\n 
";
    let prompt: &str = "$ ";

    console.background_render();
    console.print(ascii);

    console.print(prompt);

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
