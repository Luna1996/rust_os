#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(alloc_error_handler)]

pub extern crate alloc;

pub type VOID_PTR = *mut u8;

pub type EFI_GUID = u128;
pub type EFI_STATUS = usize;
pub type EFI_HANDLE = VOID_PTR;
pub type EFI_EVENT = VOID_PTR;
pub type EFI_LBA = u64;
pub type EFI_TPL = usize;
pub type EFI_MAC_ADDRESS = [u8; 32];
pub type EFI_IPv4_ADDRESS = u32;
pub type EFI_IPv6_ADDRESS = u128;
pub type EFI_IP_ADDRESS = u128;
pub type EFI_PHYSICAL_ADDRESS = u64;
pub type EFI_VIRTUAL_ADDRESS = u64;

pub mod efi_system_table;
pub mod hii_configuration_processing_and_browser_protocol;
pub mod hii_protocols;
pub mod miscellaneous_protocols;
pub mod network;
pub mod protocols;
pub mod redfish_service_support;
pub mod search_technologies;
pub mod secure_boot_and_driver_signing;
pub mod secure_technologies;
pub mod services;
pub mod user_identification;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use efi_system_table::EFI_BOOT_SERVICES;
use services::boot::EFI_MEMORY_TYPE;

pub struct Allocator<'a> {
  pub boot_services: Option<&'a EFI_BOOT_SERVICES>,
}

unsafe impl GlobalAlloc for Allocator<'_> {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let mut p: *mut u8 = ptr::null_mut();
    if let Some(bs) = self.boot_services {
      (bs.AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, layout.size(), &mut p);
    }
    p
  }

  unsafe fn dealloc(&self, p: *mut u8, _layout: Layout) {
    if let Some(bs) = self.boot_services {
      (bs.FreePool)(p);
    }
  }
}

#[global_allocator]
pub static mut ALC: Allocator = Allocator {
  boot_services: None,
};

#[alloc_error_handler]
fn alloc_panic(_: Layout) -> ! {
  loop {}
}

use efi_system_table::EFI_SYSTEM_TABLE;
pub fn init(_img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) {
  unsafe { ALC.boot_services = Some(st.BootServices) };
}
