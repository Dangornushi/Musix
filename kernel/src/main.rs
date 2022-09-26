#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate rlibc;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use kernel::console::Console;
use kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use kernel::pci::{read_class_code, read_vendor_id, scan_all_bus};
use kernel::{print, println};

fn list_pci_devices(c: &mut Console) {
    let pci_devices = scan_all_bus().unwrap();
    write!(c, "pci devices successfully scanned.");
    for dev in pci_devices.iter() {
        let vendor_id = read_vendor_id(dev.bus, dev.device, dev.function);
        let class_code = read_class_code(dev.bus, dev.device, dev.function);
        write!(
            c,
            "{}.{}.{}:, vend {:04x}, class {:08x}, head {:02x}",
            dev.bus, dev.device, dev.function, vendor_id, class_code, dev.header_type
        );
    }
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };

    let prompt: &str = "$ ";
    let mut console = Console::new(fb, mi);
    console.start();
    write!(console, "{}", prompt).unwrap();
    list_pci_devices(&mut console);

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
