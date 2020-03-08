#![no_std]
#![no_main]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(const_raw_ptr_deref, alloc_error_handler)]

#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;
mod console;
mod file;
mod memory;
mod s16;
mod video;

use alloc::{
	alloc::{GlobalAlloc, Layout},
	slice::from_raw_parts_mut,
};
use core::{
	panic::PanicInfo,
	ptr::{null, null_mut},
};
use elf::ELFFile;
use uefi::{
	efi_system_table::EFI_SYSTEM_TABLE,
	protocols::console_support::EFI_GRAPHICS_OUTPUT_PROTOCOL,
	services::{
		boot::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE},
		runtime::EFI_RESET_TYPE,
	},
	types::*,
};

//╔═══════════╗
//║  Global 	║
//╚═══════════╝
pub static mut HIMG: EFI_HANDLE = null_mut();
pub static mut ST: *const EFI_SYSTEM_TABLE = null();
pub static mut VP: *const EFI_GRAPHICS_OUTPUT_PROTOCOL = null();

//╔═══════════╗
//║ 	Entry		║
//╚═══════════╝
#[no_mangle]
unsafe fn efi_main(img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) -> EFI_STATUS {
	HIMG = img;
	ST = st;
	console::init();
	video::init();
	let root = file::init();
	let buf = file::read_file(root, "kernel");
	let _kernel_file = ELFFile::<u64>::parse(buf.as_slice()).unwrap();
	// for ph in kernel_file.phs {
	// 	print!("program type: 0x{:08X}\r\n", ph.p_type as u32);
	// }
	// for sh in kernel_file.shs {
	// 	print!(
	// 		"section: {} 0x{:0>16X}\r\n",
	// 		kernel_file.getSectionName(sh),
	// 		sh.sh_addr
	// 	);
	// }
	// Draw a green fram
	memory::init();
	// (st.RuntimeServices.ResetSystem)(EFI_RESET_TYPE::EfiResetShutdown, 0, 0, null());
	loop {}
}

//╔═══════════╗
//║ 	Panic 	║
//╚═══════════╝
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
	print!("{}", info);
	loop {}
}

//╔═══════════╗
//║ Allocator ║
//╚═══════════╝
#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
	loop {}
}

#[global_allocator]
static ALLOCATOR: _Allocator = _Allocator {};
struct _Allocator {}
unsafe impl GlobalAlloc for _Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let n = layout.size();
		let a = layout.align();
		let mut buf: *mut u8 = null_mut();
		if a < 4096 {
			((*ST).BootServices.AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n, &mut buf);
		} else {
			((*ST).BootServices.AllocatePages)(
				EFI_ALLOCATE_TYPE::AllocateAnyPages,
				EFI_MEMORY_TYPE::EfiLoaderData,
				n >> 12,
				&mut (buf as u64),
			);
		}
		buf
	}
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let n = layout.size();
		let a = layout.align();
		if a < 4096 {
			((*ST).BootServices.FreePool)(ptr);
		} else {
			((*ST).BootServices.FreePages)(ptr as u64, n >> 12);
		}
	}
}
