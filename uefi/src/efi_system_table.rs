#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::protocols::console_support::*;
use crate::services::boot::*;
use crate::*;

// EFI_IMAGE_ENTRY_POINT
// (ImageHandle:EFI_HANDLE,*SystemTable:*mut EFI_SYSTEM_TABLE)->EFI_STATUS

// 4.2 EFI Table Header
#[repr(C)]
pub struct EFI_TABLE_HEADER {
  pub Signature: u64,
  pub Revision: u32,
  pub HeaderSize: u32,
  pub CRC32: u32,
  pub Reserved: u32,
}

// 4.3 EFI System Table
pub const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;
pub const EFI_2_80_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (80));
pub const EFI_2_70_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (70));
pub const EFI_2_60_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (60));
pub const EFI_2_50_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (50));
pub const EFI_2_40_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (40));
pub const EFI_2_31_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (31));
pub const EFI_2_30_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (30));
pub const EFI_2_20_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (20));
pub const EFI_2_10_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (10));
pub const EFI_2_00_SYSTEM_TABLE_REVISION: u32 = ((2 << 16) | (00));
pub const EFI_1_10_SYSTEM_TABLE_REVISION: u32 = ((1 << 16) | (10));
pub const EFI_1_02_SYSTEM_TABLE_REVISION: u32 = ((1 << 16) | (02));
pub const EFI_SPECIFICATION_VERSION: u32 = EFI_SYSTEM_TABLE_REVISION;
pub const EFI_SYSTEM_TABLE_REVISION: u32 = EFI_2_80_SYSTEM_TABLE_REVISION;

#[repr(C)]
pub struct EFI_SYSTEM_TABLE<'a> {
  pub Hdr: EFI_TABLE_HEADER,
  pub FirmwareVendor: *const u16,
  pub FirmwareRevision: u32,
  pub ConsoleInHandle: EFI_HANDLE,
  pub ConIn: &'a EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
  pub ConsoleOutHandle: EFI_HANDLE,
  pub ConOut: &'a EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL<'a>,
  pub StandardErrorHandle: EFI_HANDLE,
  pub StdErr: &'a EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL<'a>,
  pub RuntimeServices: &'a EFI_RUNTIME_SERVICES,
  pub BootServices: &'a EFI_BOOT_SERVICES,
  pub NumberOfTableEntries: usize,
  pub ConfigurationTable: &'a EFI_CONFIGURATION_TABLE,
}

// 4.4 EFI Boot Services Table
pub const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;
pub const EFI_BOOT_SERVICES_REVISION: u32 = EFI_SPECIFICATION_VERSION;

#[repr(C)]
pub struct EFI_BOOT_SERVICES {
  pub Hdr: EFI_TABLE_HEADER,
  // Task Priority Services
  pub RaiseTPL: EFI_RAISE_TPL,
  pub RestoreTPL: EFI_RESTORE_TPL,
  // Memory Services
  pub AllocatePages: EFI_ALLOCATE_PAGES,
  pub FreePages: EFI_FREE_PAGES,
  pub GetMemoryMap: EFI_GET_MEMORY_MAP,
  pub AllocatePool: EFI_ALLOCATE_POOL,
  pub FreePool: EFI_FREE_POOL,
  // Event & Timer Services
  pub CreateEvent: EFI_CREATE_EVENT,
  pub SetTimer: EFI_SET_TIMER,
  pub WaitForEvent: EFI_WAIT_FOR_EVENT,
  pub SignalEvent: EFI_SIGNAL_EVENT,
  pub CloseEvent: EFI_CLOSE_EVENT,
  pub CheckEvent: EFI_CHECK_EVENT,
  // Protocol Handler Services
  pub InstallProtocolInterface: EFI_INSTALL_PROTOCOL_INTERFACE,
  pub ReinstallProtocolInterface: EFI_REINSTALL_PROTOCOL_INTERFACE,
  pub UninstallProtocolInterface: EFI_UNINSTALL_PROTOCOL_INTERFACE,
  pub HandleProtocol: EFI_HANDLE_PROTOCOL,
  pub Reserved: VOID_PTR,
  pub RegisterProtocolNotify: EFI_REGISTER_PROTOCOL_NOTIFY,
  pub LocateHandle: EFI_LOCATE_HANDLE,
  pub LocateDevicePath: EFI_LOCATE_DEVICE_PATH,
  pub InstallConfigurationTable: EFI_INSTALL_CONFIGURATION_TABLE,
  // Image Services
  pub LoadImage: EFI_IMAGE_LOAD,
  pub StartImage: EFI_IMAGE_START,
  pub Exit: EFI_EXIT,
  pub UnloadImage: EFI_IMAGE_UNLOAD,
  pub ExitBootServices: EFI_EXIT_BOOT_SERVICES,
  // Miscellaneous Services
  pub GetNextMonotonicCount: EFI_GET_NEXT_MONOTONIC_COUNT,
  pub Stall: EFI_STALL,
  pub SetWatchdogTimer: EFI_SET_WATCHDOG_TIMER,
  // DriverSupport Services
  pub ConnectControlle: EFI_CONNECT_CONTROLLER,
  pub DisconnectControlle: EFI_DISCONNECT_CONTROLLER,
  // Open and Close Protocol Services
  pub OpenProtocol: EFI_OPEN_PROTOCOL,
  pub CloseProtocol: EFI_CLOSE_PROTOCOL,
  pub OpenProtocolInformation: EFI_OPEN_PROTOCOL_INFORMATION,
  // Library Services
  pub ProtocolsPerHandle: EFI_PROTOCOLS_PER_HANDLE,
  pub LocateHandleBuffer: EFI_LOCATE_HANDLE_BUFFER,
  pub LocateProtocol: EFI_LOCATE_PROTOCOL,
  pub InstallMultipleProtocolInterfaces: EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
  pub UninstallMultipleProtocolInterfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,
  // 32-bit CRC Services
  pub CalculateCrc32: EFI_CALCULATE_CRC32,
  // Miscellaneous Services
  pub CopyMem: EFI_COPY_MEM,
  pub SetMem: EFI_SET_MEM,
  pub CreateEventEx: EFI_CREATE_EVENT_EX,
}

// 4.5 EFI Runtime Services Table
use services::runtime::*;
pub const EFI_RUNTIME_SERVICES_SIGNATURE: u64 = 0x56524553544e5552;
pub const EFI_RUNTIME_SERVICES_REVISION: u32 = EFI_SPECIFICATION_VERSION;

#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
  pub Hdr: EFI_TABLE_HEADER,
  // Time Services
  pub GetTime: EFI_GET_TIME,
  pub SetTime: EFI_SET_TIME,
  pub GetWakeupTime: EFI_GET_WAKEUP_TIME,
  pub SetWakeupTime: EFI_SET_WAKEUP_TIME,
  // Virtual Memory Services
  pub SetVirtualAddressMap: EFI_SET_VIRTUAL_ADDRESS_MAP,
  pub ConvertPointer: EFI_CONVERT_POINTER,
  // Variable Services
  pub GetVariable: EFI_GET_VARIABLE,
  pub GetNextVariableName: EFI_GET_NEXT_VARIABLE_NAME,
  pub SetVariable: EFI_SET_VARIABLE,
  // Miscellaneous Services
  pub GetNextHighMonotonicCount: EFI_GET_NEXT_HIGH_MONO_COUNT,
  pub ResetSystem: EFI_RESET_SYSTEM,
  // UEFI 2.0 Capsule Services
  pub UpdateCapsule: EFI_UPDATE_CAPSULE,
  pub QueryCapsuleCapabilities: EFI_QUERY_CAPSULE_CAPABILITIES,
  // Miscellaneous UEFI 2.0 Service
  pub QueryVariableInfo: EFI_QUERY_VARIABLE_INFO,
}

// 4.6 EFI Configuration Table & Properties Table
#[repr(C)]
pub struct EFI_CONFIGURATION_TABLE {
  pub VendorGuid: EFI_GUID,
  pub VendorTable: VOID_PTR,
}

pub const EFI_ACPI_TABLE_GUID: EFI_GUID = 0x8868e871e4f111d3bc220080c73c8881;
pub const EFI_ACPI_20_TABLE_GUID: EFI_GUID = EFI_ACPI_TABLE_GUID;
pub const ACPI_TABLE_GUID: EFI_GUID = 0xeb9d2d302d8811d39a160090273fc14d;
pub const ACPI_10_TABLE_GUID: EFI_GUID = ACPI_TABLE_GUID;

pub const EFI_PROPERTIES_TABLE_VERSION: u32 = 0x00010000;

#[repr(C)]
pub struct EFI_PROPERTIES_TABLE {
  pub Version: u32,
  pub Length: u32,
  pub MemoryProtectionAttribute: u64,
}

pub const EFI_MEMORY_ATTRIBUTES_TABLE_GUID: EFI_GUID = 0xdcfa911d26eb469fa22038b7dc461220;

#[repr(C)]
pub struct EFI_MEMORY_ATTRIBUTES_TABLE {
  pub Version: u32,
  pub NumberOfEntries: u32,
  pub DescriptorSize: u32,
  pub Reserved: u32,
}
