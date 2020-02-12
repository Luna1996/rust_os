#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::alloc::vec::*;
use uefi::alloc::*;
use uefi::efi_system_table::*;
use uefi::protocols::console_support::*;
use uefi::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

fn s16(s: &str) -> Vec<u16> {
  let mut s_: Vec<u16> = vec![0; s.len() + 1];
  for (i, val) in s.encode_utf16().enumerate() {
    s_[i] = val;
  }
  s_
}

#[no_mangle]
fn efi_main(img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) -> EFI_STATUS {
  let stdout: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL = st.ConOut;
  let bs: &EFI_BOOT_SERVICES = st.BootServices;
  init(img, st);
  (stdout.Reset)(stdout, true);
  (stdout.OutputString)(
    stdout,
    s16(format!("Hello {} World!", "fuck").as_str()).as_ref(),
  );
  loop {
    (stdout.EnableCursor)(stdout, true);
    (bs.Stall)(500000);
    (stdout.EnableCursor)(stdout, false);
    (bs.Stall)(500000);
  }
}
