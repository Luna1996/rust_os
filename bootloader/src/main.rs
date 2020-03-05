#![no_std]
#![no_main]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(const_raw_ptr_deref, alloc_error_handler)]

#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;
mod console;
mod video;

use uefi::types::*;

//╔═══════════╗
//║  Global 	║
//╚═══════════╝
use uefi::efi_system_table::EFI_SYSTEM_TABLE;
pub static mut ST: Option<&EFI_SYSTEM_TABLE> = None;

//╔═══════════╗
//║ 	Entry		║
//╚═══════════╝
#[no_mangle]
unsafe fn efi_main(img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) -> EFI_STATUS {
	// setup global variables
	ST = Some(st);
	// set text mode to max suport mode
	console::init(st);
	video::init(st);
	loop {}
}

//╔═══════════╗
//║ 	Panic 	║
//╚═══════════╝
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

//╔═══════════╗
//║ Allocator ║
//╚═══════════╝
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use uefi::services::boot::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE};

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
			(ST.unwrap().BootServices.AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n, &mut buf);
		} else {
			(ST.unwrap().BootServices.AllocatePages)(
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
			(ST.unwrap().BootServices.FreePool)(ptr);
		} else {
			(ST.unwrap().BootServices.FreePages)(ptr as u64, n >> 12);
		}
	}
}
