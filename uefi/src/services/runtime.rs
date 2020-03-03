use crate::services::boot::EFI_MEMORY_TYPE;
use crate::types::*;

// 8.1 Runtime Services Rules and Restrictions
pub const EFI_RT_SUPPORTED_GET_TIME: u16 = 0x0001;
pub const EFI_RT_SUPPORTED_SET_TIME: u16 = 0x0002;
pub const EFI_RT_SUPPORTED_GET_WAKEUP_TIME: u16 = 0x0004;
pub const EFI_RT_SUPPORTED_SET_WAKEUP_TIME: u16 = 0x0008;
pub const EFI_RT_SUPPORTED_GET_VARIABLE: u16 = 0x0010;
pub const EFI_RT_SUPPORTED_GET_NEXT_VARIABLE_NAME: u16 = 0x0020;
pub const EFI_RT_SUPPORTED_SET_VARIABLE: u16 = 0x0040;
pub const EFI_RT_SUPPORTED_SET_VIRTUAL_ADDRESS_MAP: u16 = 0x0080;
pub const EFI_RT_SUPPORTED_CONVERT_POINTER: u16 = 0x0100;
pub const EFI_RT_SUPPORTED_GET_NEXT_HIGH_MONOTONIC_COUNT: u16 = 0x0200;
pub const EFI_RT_SUPPORTED_RESET_SYSTEM: u16 = 0x0400;
pub const EFI_RT_SUPPORTED_UPDATE_CAPSULE: u16 = 0x0800;
pub const EFI_RT_SUPPORTED_QUERY_CAPSULE_CAPABILITIES: u16 = 0x1000;
pub const EFI_RT_SUPPORTED_QUERY_VARIABLE_INFO: u16 = 0x2000;

// 8.2 Variable Services
// EFI_GET_VARIABLE
pub type EFI_GET_VARIABLE = extern "C" fn(
  VariableName: *const u16,     // IN
  VendorGuid: &EFI_GUID,        // IN
  Attributes: Option<&mut u32>, // OUT
  DataSize: &mut usize,         // IN OUT
  Data: Option<*mut u8>,        // OUT
) -> EFI_STATUS;
// Attributes
pub const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;
pub const EFI_VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002;
pub const EFI_VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004;
pub const EFI_VARIABLE_HARDWARE_ERROR_RECORD: u32 = 0x00000008;
pub const EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000010;
pub const EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000020;
pub const EFI_VARIABLE_APPEND_WRITE: u32 = 0x00000040;
pub const EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS: u32 = 0x00000080;

// EFI_GET_NEXT_VARIABLE_NAME
pub type EFI_GET_NEXT_VARIABLE_NAME = extern "C" fn(
  VariableNameSize: &mut usize, // IN OUT
  VariableName: *mut u16,       // IN OUT
  VendorGuid: &mut EFI_GUID,    // IN OUT
) -> EFI_STATUS;

// EFI_SET_VARIABLE
pub type EFI_SET_VARIABLE = extern "C" fn(
  VariableName: *const u16, // IN
  VendorGuid: &EFI_GUID,    // IN
  Attributes: u32,          // IN
  DataSize: usize,          // IN
  Data: *const u8,          // IN
) -> EFI_STATUS;

// EFI_QUERY_VARIABLE_INFO
pub type EFI_QUERY_VARIABLE_INFO = extern "C" fn(
  Attributes: u32,                        // IN
  MaximumVariableStorageSize: &mut u64,   // OUT
  RemainingVariableStorageSize: &mut u64, // OUT
  MaximumVariableSize: &mut u64,          // OUT
) -> EFI_STATUS;

// 8.3 Time Services
// EFI_GET_TIME()
pub type EFI_GET_TIME = extern "C" fn(
  Time: &mut EFI_TIME,                              // OUT
  Capabilities: Option<&mut EFI_TIME_CAPABILITIES>, // OUT
) -> EFI_STATUS;
//EFI_TIME
#[repr(C)]
pub struct EFI_TIME {
  pub Year: u16,       // 1900 – 9999
  pub Month: u8,       // 1 – 12
  pub Day: u8,         // 1 – 31
  pub Hour: u8,        // 0 – 23
  pub Minute: u8,      // 0 – 59
  pub Second: u8,      // 0 – 59
  pub Pad1: u8,        //
  pub Nanosecond: u32, // 0 – 999,999,999
  pub TimeZone: u16,   // -1440 to 1440 or 2047
  pub Daylight: u8,
  pub Pad2: u8,
}
// EFI_TIME.Daylight
pub const EFI_TIME_ADJUST_DAYLIGHT: u8 = 0x01;
pub const EFI_TIME_IN_DAYLIGHT: u8 = 0x02;
// EFI_TIME.TimeZone
pub const EFI_UNSPECIFIED_TIMEZONE: u16 = 0x07FF;

// EFI_TIME_CAPABILITIES
pub struct EFI_TIME_CAPABILITIES {
  pub Resolution: u32,
  pub Accuracy: u32,
  pub SetsToZero: bool,
}

// EFI_SET_TIME()
pub type EFI_SET_TIME = extern "C" fn(
  Time: &EFI_TIME, // IN
) -> EFI_STATUS;

// EFI_GET_WAKEUP_TIME()
pub type EFI_GET_WAKEUP_TIME = extern "C" fn(
  Enabled: &mut bool,  // OUT
  Pending: &mut bool,  // OUT
  Time: &mut EFI_TIME, // OUT
) -> EFI_STATUS;

// EFI_SET_WAKEUP_TIME()
pub type EFI_SET_WAKEUP_TIME = extern "C" fn(
  Enable: bool,            // IN
  Time: Option<&EFI_TIME>, // IN
) -> EFI_STATUS;

// 8.4 Virtual Memory Services
use crate::services::boot::EFI_MEMORY_DESCRIPTOR;
// EFI_SET_VIRTUAL_ADDRESS_MAP()
pub type EFI_SET_VIRTUAL_ADDRESS_MAP = extern "C" fn(
  MemoryMapSize: usize,               // IN
  DescriptorSize: usize,              // IN
  DescriptorVersion: u32,             // IN
  VirtualMap: &EFI_MEMORY_DESCRIPTOR, // IN
) -> EFI_STATUS;

// EFI_CONVERT_POINTER()
pub type EFI_CONVERT_POINTER = extern "C" fn(
  DebugDisposition: usize,   // IN
  Address: *const *const u8, // IN
) -> EFI_STATUS;

// 8.5 Miscellaneous Runtime Services
// EFI_RESET_SYSTEM()
pub type EFI_RESET_SYSTEM = extern "C" fn(
  ResetType: EFI_RESET_TYPE,    // IN
  ResetStatus: EFI_STATUS,      // IN
  DataSize: usize,              // IN
  ResetData: Option<*const u8>, // IN
);
// EFI_RESET_TYPE
#[repr(C)]
pub enum EFI_RESET_TYPE {
  EfiResetCold,
  EfiResetWarm,
  EfiResetShutdown,
  EfiResetPlatformSpecific,
}

// EFI_GET_NEXT_HIGH_MONO_COUNT()
pub type EFI_GET_NEXT_HIGH_MONO_COUNT = extern "C" fn(
  HighCount: &mut u32, // OUT
) -> EFI_STATUS;

// EFI_UPDATE_CAPSULE()
pub type EFI_UPDATE_CAPSULE = extern "C" fn(
  CapsuleHeaderArray: &&EFI_CAPSULE_HEADER,        // IN
  CapsuleCount: usize,                             // IN
  ScatterGatherList: Option<EFI_PHYSICAL_ADDRESS>, // IN
) -> EFI_STATUS;
#[repr(C)]
pub struct EFI_CAPSULE_BLOCK_DESCRIPTOR {
  pub Length: u64,
  pub Union: EFI_DATA_BLOCK_OR_CONTINUATION_POINTER,
}
#[repr(C)]
pub union EFI_DATA_BLOCK_OR_CONTINUATION_POINTER {
  pub DataBlock: EFI_PHYSICAL_ADDRESS,
  pub ContinuationPointer: EFI_PHYSICAL_ADDRESS,
}
#[repr(C)]
pub struct EFI_CAPSULE_HEADER {
  pub CapsuleGuid: EFI_GUID,
  pub HeaderSize: u32,
  pub Flags: u32,
  pub CapsuleImageSize: u32,
}
// Flags
pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: u32 = 0x00010000;
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: u32 = 0x00020000;
pub const CAPSULE_FLAGS_INITIATE_RESET: u32 = 0x00040000;
#[repr(C)]
pub struct EFI_CAPSULE_TABLE {
  pub CapsuleArrayNumber: u32,
  pub CapsulePtr: [*mut u8; 1],
}

// EFI_MEMORY_RANGE_CAPSULE_GUID
pub const EFI_MEMORY_RANGE_CAPSULE_GUID: EFI_GUID = 0x0DE9F0EC88B6428F977A258F1D0E5E72;
#[repr(C)]
pub struct EFI_MEMORY_RANGE {
  pub Address: EFI_PHYSICAL_ADDRESS,
  pub Length: u64,
}
#[repr(C)]
pub struct EFI_MEMORY_RANGE_CAPSULE<'a> {
  pub Header: EFI_CAPSULE_HEADER,
  pub OsRequestedMemoryType: EFI_MEMORY_TYPE,
  pub NumberOfMemoryRanges: u64,
  pub MemoryRanges: &'a [EFI_MEMORY_RANGE],
}

// EFI_QUERY_CAPSULE_CAPABILITIES()
pub type EFI_QUERY_CAPSULE_CAPABILITIES = extern "C" fn(
  CapsuleHeaderArray: &[&EFI_CAPSULE_HEADER], // IN
  CapsuleCount: usize,                        // IN
  MaximumCapsuleSize: &mut u64,               // OUT
  ResetType: &mut EFI_RESET_TYPE,             // OUT
) -> EFI_STATUS;

// EFI_CAPSULE_REPORT_GUID
pub const EFI_CAPSULE_REPORT_GUID: EFI_GUID = 0x39B68C46F7FB441BB6EC16B0F69821F3;
#[repr(C)]
pub struct EFI_CAPSULE_RESULT_VARIABLE_HEADER {
  pub VariableTotalSize: u32,
  _Reserved: u32,
  pub CapsuleGuid: EFI_GUID,
  pub CapsuleProcessed: EFI_TIME,
  pub CapsuleStatus: EFI_STATUS,
}
