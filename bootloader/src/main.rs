#![no_std]
#![no_main]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(const_raw_ptr_deref, alloc_error_handler)]
#[macro_use]
extern crate alloc;
#[macro_use]
mod std;

use core::panic::PanicInfo;
use std::{init, s16, s8};
use uefi::efi_system_table::EFI_SYSTEM_TABLE;
use uefi::types::{EFI_HANDLE, EFI_STATUS};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
unsafe fn efi_main(_img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) -> EFI_STATUS {
	let stdout = st.ConOut;
	let boot = st.BootServices;
	init(st);
	(stdout.ClearScreen)(stdout);
	efi_print!("FirmwareVendor: {}", s8(&st.FirmwareVendor));
	loop {
		(stdout.EnableCursor)(stdout, true);
		(boot.Stall)(500000);
		(stdout.EnableCursor)(stdout, false);
		(boot.Stall)(500000);
	}
}
