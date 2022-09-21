#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::arch::asm;
use core::mem;
use core::fmt::Write;
use log::info;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::table::boot::MemoryDescriptor;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    writeln!(&mut st.stdout(), "Hello, Musix!!\n").unwrap();

    loop {
        unsafe{
            asm!("hlt");
        }
    }

    Status::SUCCESS
}
