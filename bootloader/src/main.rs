#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use alloc::slice;
use alloc::string::String;
use alloc::vec::Vec;
use core::arch::asm;
use core::fmt::Write;
use core::mem;
use goblin::elf::{program_header, Elf};
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop;
use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::FileInfo;
use uefi::proto::media::file::RegularFile;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::MemoryDescriptor;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::runtime::ResetType;
use uefi::CStr16;
use uefi_services::system_table;

const KERNEL_BASE_ADDR: usize = 0x100000;
const EFI_PAGE_SIZE: usize = 0x1000;

// load elf file (kernel file)
fn entry_kernel(boot_services: &BootServices, buf: Vec<u8>) -> usize {
    let elf = Elf::parse(&buf).unwrap();

    let mut dest_start = usize::MAX;
    let mut dest_end = 0;

    for ph in elf.program_headers.iter() {
        log::info!(
            "program_header: {} {} {} {} {}",
            program_header::pt_to_str(ph.p_type),
            ph.p_offset,
            ph.p_vaddr,
            ph.p_paddr,
            ph.p_memsz
        );

        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        dest_start = dest_start.min(ph.p_paddr as usize);
        dest_end = dest_end.max(ph.p_paddr + ph.p_memsz);
    }

    boot_services
        .allocate_pages(
            AllocateType::Address(dest_start),
            MemoryType::LOADER_DATA,
            (dest_end as usize - dest_start as usize + EFI_PAGE_SIZE - 1) / EFI_PAGE_SIZE,
        )
        .unwrap();

    for ph in elf.program_headers.iter() {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        let dest = unsafe { slice::from_raw_parts_mut(ph.p_paddr as *mut u8, ph.p_memsz as usize) };
        dest[..(ph.p_filesz as usize)].copy_from_slice(
            &buf[(ph.p_offset as usize)..(ph.p_offset as usize + ph.p_filesz as usize)],
        );
        dest[(ph.p_filesz as usize)..].fill(0);
    }
    elf.entry as usize
}

// Load kernel
fn load_kernel(boot_services: &BootServices, image: Handle) -> usize {
    // open dir
    let loaded_image = unsafe {
        boot_services
            .handle_protocol::<LoadedImage>(image)
            .unwrap()
            .get()
    };
    let device = unsafe { (*loaded_image).device() };
    let file_system = unsafe {
        boot_services
            .handle_protocol::<SimpleFileSystem>(device)
            .unwrap()
            .get()
    };
    let mut root_dir = unsafe { (*file_system).open_volume().unwrap() };

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
    file.close();
    entry_kernel(&boot_services, buf)
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

    // elf fileのエントリーポイント
    let elf_entry = load_kernel(st.boot_services(), image);

    // GOP
    let gop = unsafe {
        st.boot_services()
            .locate_protocol::<gop::GraphicsOutput>()
            .expect_err("Your computer does not support Graphics Output Protocol!")
    };

    //unsafe { &mut *gop.get() }

    // エントリーポイント先のアドレスの関数を作成
    let Musix: extern "sysv64" fn() = unsafe { mem::transmute(elf_entry) };

    // カーネル実行(胸熱)
    Musix();

    // UEFIの全機能を停止
    st.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None)
}
