use crate::efi_system_table::EFI_SYSTEM_TABLE;
use crate::services::boot::EFI_MEMORY_TYPE;
use crate::types::*;
// EFI_LOADED_IMAGE_PROTOCOL
pub const EFI_LOADED_IMAGE_PROTOCOL_REVISION: u16 = 0x1000;

struct EFI_DEVICE_PATH_PROTOCOL;

#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL<'a> {
  pub Revision: u32,
  pub ParentHandle: EFI_HANDLE,
  pub SystemTable: &'a EFI_SYSTEM_TABLE<'a>,
  // Source location of the image
  pub DeviceHandle: EFI_HANDLE,
  FilePath: &'a EFI_DEVICE_PATH_PROTOCOL,
  Reserved: *const u8,
  // Image’s load options
  pub LoadOptionsSize: u32,
  pub LoadOptions: *mut u8,
  // Location where image was loaded
  pub ImageBase: *mut u8,
  pub ImageSize: u64,
  pub ImageCodeType: EFI_MEMORY_TYPE,
  pub ImageDataType: EFI_MEMORY_TYPE,
  pub Unload: EFI_IMAGE_UNLOAD,
}

// EFI_LOADED_IMAGE_PROTOCOL.Unload()
pub type EFI_IMAGE_UNLOAD = extern "C" fn(
  ImageHandle: EFI_HANDLE, // IN
) -> EFI_STATUS;
