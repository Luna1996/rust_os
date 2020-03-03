#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod types {
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
}

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
