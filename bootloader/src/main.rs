#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use core::arch::asm;
use core::fmt::Write;
use core::mem;
use log::info;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::FileInfo;
use uefi::proto::media::file::RegularFile;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::MemoryDescriptor;
use uefi::table::runtime::ResetType;
use uefi::CStr16;

// Load kernel
fn load_kernel(boot_services: &BootServices, image: Handle) {
    unsafe {
        let loaded_image = boot_services
            .handle_protocol::<LoadedImage>(image)
            .unwrap()
            .get();
        let device = unsafe { (*loaded_image).device() };
        let file_system = boot_services
            .handle_protocol::<SimpleFileSystem>(device)
            .unwrap()
            .get();
        let mut root_dir = unsafe { (*file_system).open_volume().unwrap() };
        let mut buf = vec![0; 4096];

        // RegularFile取得
        let mut cstr_buf = [0u16; 32];
        let cstr_file_name = CStr16::from_str_with_buf("kernel.elf", &mut cstr_buf).unwrap();
        let file_handle = root_dir
            .open(cstr_file_name, FileMode::Read, FileAttribute::empty())
            .unwrap();
        let mut file = unsafe { RegularFile::new(file_handle) };
        // サイズ取得
        let file_size = file.get_boxed_info::<FileInfo>().unwrap().file_size() as usize;
        // バッファへの読み込み
        let mut buf = vec![0; file_size];
        file.read(&mut buf);
        log::info!("{:?}", buf);
    };
}

// Get memory map -> memorymap vec
fn get_memory_map(boot_services: &BootServices) -> Vec<MemoryDescriptor> {
    let mapSize = boot_services.memory_map_size().map_size;
    let mut mapBuffer = vec![0; mapSize * 8];
    info!("mapBuffer size: {}", mapSize);

    let (mapKey, descItr) = boot_services.memory_map(&mut mapBuffer).unwrap();

    descItr.copied().collect::<alloc::vec::Vec<_>>()
}

#[entry]
fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).unwrap();

    writeln!(&mut st.stdout(), "Hello, Musix!!\n").unwrap();

    let mut descriptors = get_memory_map(st.boot_services());

    descriptors.iter().for_each(|descriptor| {
        info!(
            "{:?}, {}, {}, {}",
            descriptor.ty, descriptor.phys_start, descriptor.virt_start, descriptor.page_count
        );
    });

    load_kernel(st.boot_services(), image);

    loop {
        unsafe {
            asm!("hlt");
        }
    }

    // return exit status
    // Status::SUCCESS
}
