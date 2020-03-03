#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::efi_system_table::EFI_SYSTEM_TABLE;
use uefi::types::{EFI_HANDLE, EFI_STATUS};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
fn efi_main(_img: EFI_HANDLE, st: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
	let stdout = st.ConOut;
	let boot = st.BootServices;
	(stdout.ClearScreen)(stdout);
	loop {
		(stdout.EnableCursor)(stdout, true);
		(boot.Stall)(500000);
		(stdout.EnableCursor)(stdout, false);
		(boot.Stall)(500000);
	}
}
