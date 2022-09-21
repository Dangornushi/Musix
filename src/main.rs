#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::arch::asm;
use core::fmt::Write;
use core::mem;
use log::info;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::table::boot::MemoryDescriptor;
use uefi::table::runtime::ResetType;

#[entry]
fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    writeln!(&mut st.stdout(), "Hello, Musix!!\n").unwrap();

    let mapSize = st.boot_services().memory_map_size().map_size;
    let mut mapBuffer = vec![0; mapSize * 8];
    info!("mapBuffer size: {}", mapSize);

    let (mapKey, descItr) = st.boot_services().memory_map(&mut mapBuffer).unwrap();

    let descriptors = descItr.copied().collect::<alloc::vec::Vec<_>>();
    descriptors.iter().for_each(|descriptor| {
        info!(
            "{:?}, {}, {}, {}",
            descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count
        );
    });

    loop {
        unsafe {
            asm!("hlt");
        }
    }

    Status::SUCCESS
}
