#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;

// EFI_BOOT_SERVICES.CreateEvent()
pub type EFI_CREATE_EVENT = extern "C" fn(
  Type: u32,                                // IN
  NotifyTpl: EFI_TPL,                       // IN
  NotifyFunction: Option<EFI_EVENT_NOTIFY>, // IN
  NotifyContext: Option<VOID_PTR>,          // IN
  Event: &mut EFI_EVENT,                    // OUT
) -> EFI_STATUS;

pub const EVT_TIMER: u32 = 0x80000000;
pub const EVT_RUNTIME: u32 = 0x40000000;
pub const EVT_NOTIFY_WAIT: u32 = 0x00000100;
pub const EVT_NOTIFY_SIGNAL: u32 = 0x00000200;
pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: u32 = 0x00000201;
pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: u32 = 0x60000202;

pub type EFI_EVENT_NOTIFY = extern "C" fn(
  Event: EFI_EVENT,  // IN
  Context: VOID_PTR, // IN
);

// EFI_BOOT_SERVICES.CreateEventEx()
pub type EFI_CREATE_EVENT_EX = extern "C" fn(
  Type: u32,                                // IN
  NotifyTpl: EFI_TPL,                       // IN
  NotifyFunction: Option<EFI_EVENT_NOTIFY>, // IN
  NotifyContext: Option<VOID_PTR>,          // IN
  EventGroup: Option<&EFI_GUID>,            // IN
  Event: &mut EFI_EVENT,                    // OUT
) -> EFI_STATUS;

pub const EFI_EVENT_GROUP_EXIT_BOOT_SERVICES: EFI_GUID = 0x27abf055b1b84c268048748f37baa2df;
pub const EFI_EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE: EFI_GUID = 0x13fa7698c83149c787ea8f43fcc25196;
pub const EFI_EVENT_GROUP_MEMORY_MAP_CHANGE: EFI_GUID = 0x78bee926692f48fd9edb01422ef0d7ab;
pub const EFI_EVENT_GROUP_READY_TO_BOOT: EFI_GUID = 0x7ce88fb34bd7467987a8a8d8dee50d2b;
pub const EFI_EVENT_GROUP_RESET_SYSTEM: EFI_GUID = 0x62da6a5613fb485aa8daa3dd7912cb6b;

// EFI_BOOT_SERVICES.CloseEvent()
pub type EFI_CLOSE_EVENT = extern "C" fn(
  Event: EFI_EVENT, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.SignalEvent()
pub type EFI_SIGNAL_EVENT = extern "C" fn(
  Event: EFI_EVENT, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.WaitForEvent()
pub type EFI_WAIT_FOR_EVENT = extern "C" fn(
  NumberOfEvents: usize, // IN
  Event: &EFI_EVENT,     // IN
  Index: &mut usize,     // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.CheckEvent()
pub type EFI_CHECK_EVENT = extern "C" fn(
  Event: EFI_EVENT, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.SetTimer()
pub type EFI_SET_TIMER = extern "C" fn(
  Event: EFI_EVENT,      // IN
  Type: EFI_TIMER_DELAY, // IN
  TriggerTime: u64,      // IN
) -> EFI_STATUS;

#[repr(C)]
pub enum EFI_TIMER_DELAY {
  TimerCancel,
  TimerPeriodic,
  TimerRelative,
}

// EFI_BOOT_SERVICES.RaiseTPL()
pub type EFI_RAISE_TPL = extern "C" fn(
  NewTpl: EFI_TPL, // IN
) -> EFI_TPL;

pub const TPL_APPLICATION: EFI_TPL = 4;
pub const TPL_CALLBACK: EFI_TPL = 8;
pub const TPL_NOTIFY: EFI_TPL = 16;
pub const TPL_HIGH_LEVEL: EFI_TPL = 31;

// EFI_BOOT_SERVICES.RestoreTPL()
pub type EFI_RESTORE_TPL = extern "C" fn(
  OldTpl: EFI_TPL, // IN
);

// EFI_BOOT_SERVICES.AllocatePages()
pub type EFI_ALLOCATE_PAGES = extern "C" fn(
  Type: EFI_ALLOCATE_TYPE,           // IN
  MemoryType: EFI_MEMORY_TYPE,       // IN
  Pages: usize,                      // IN
  Memory: &mut EFI_PHYSICAL_ADDRESS, // IN OUT
) -> EFI_STATUS;

#[repr(C)]
pub enum EFI_ALLOCATE_TYPE {
  AllocateAnyPages,
  AllocateMaxAddress,
  AllocateAddress,
  MaxAllocateType,
}

#[repr(C)]
pub enum EFI_MEMORY_TYPE {
  EfiReservedMemoryType,
  EfiLoaderCode,
  EfiLoaderData,
  EfiBootServicesCode,
  EfiBootServicesData,
  EfiRuntimeServicesCode,
  EfiRuntimeServicesData,
  EfiConventionalMemory,
  EfiUnusableMemory,
  EfiACPIReclaimMemory,
  EfiACPIMemoryNVS,
  EfiMemoryMappedIO,
  EfiMemoryMappedIOPortSpace,
  EfiPalCode,
  EfiPersistentMemory,
  EfiMaxMemoryType,
}

// EFI_BOOT_SERVICES.FreePages()
pub type EFI_FREE_PAGES = extern "C" fn(
  Memory: EFI_PHYSICAL_ADDRESS, // IN
  Pages: usize,                 // IN
);

// EFI_BOOT_SERVICES.GetMemoryMap()
pub type EFI_GET_MEMORY_MAP = extern "C" fn(
  MemoryMapSize: &mut usize,             // IN OUT
  MemoryMap: &mut EFI_MEMORY_DESCRIPTOR, // IN OUT
  MapKey: &mut usize,                    // OUT
  DescriptorSize: &mut usize,            // OUT
  DescriptorVersion: &mut u32,           // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
  Type: u32,
  PhysicalStart: EFI_PHYSICAL_ADDRESS,
  VirtualStart: EFI_VIRTUAL_ADDRESS,
  NumberOfPages: u64,
  Attribute: u64,
}

pub const EFI_MEMORY_UC: u64 = 0x0000000000000001;
pub const EFI_MEMORY_WC: u64 = 0x0000000000000002;
pub const EFI_MEMORY_WT: u64 = 0x0000000000000004;
pub const EFI_MEMORY_WB: u64 = 0x0000000000000008;
pub const EFI_MEMORY_UCE: u64 = 0x0000000000000010;
pub const EFI_MEMORY_WP: u64 = 0x0000000000001000;
pub const EFI_MEMORY_RP: u64 = 0x0000000000002000;
pub const EFI_MEMORY_XP: u64 = 0x0000000000004000;
pub const EFI_MEMORY_NV: u64 = 0x0000000000008000;
pub const EFI_MEMORY_MORE_RELIABLE: u64 = 0x0000000000010000;
pub const EFI_MEMORY_RO: u64 = 0x0000000000020000;
pub const EFI_MEMORY_SP: u64 = 0x0000000000040000;
pub const EFI_MEMORY_CPU_CRYPTO: u64 = 0x0000000000080000;
pub const EFI_MEMORY_RUNTIME: u64 = 0x8000000000000000;

pub const EFI_MEMORY_DESCRIPTOR_VERSION: u32 = 1;

// EFI_BOOT_SERVICES.AllocatePool()
pub type EFI_ALLOCATE_POOL = extern "C" fn(
  PoolType: EFI_MEMORY_TYPE, // IN
  Size: usize,               // IN
  Buffer: &mut VOID_PTR,     // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.FreePool()
pub type EFI_FREE_POOL = extern "C" fn(
  Buffer: VOID_PTR, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.InstallProtocolInterface()
pub type EFI_INSTALL_PROTOCOL_INTERFACE = extern "C" fn(
  Handle: &mut EFI_HANDLE,           // IN OUT
  Protocol: &EFI_GUID,               // IN
  InterfaceType: EFI_INTERFACE_TYPE, // IN
  Interface: VOID_PTR,               // IN
) -> EFI_STATUS;

#[repr(C)]
pub enum EFI_INTERFACE_TYPE {
  EFI_NATIVE_INTERFACE,
}

// EFI_BOOT_SERVICES.UninstallProtocolInterface()
pub type EFI_UNINSTALL_PROTOCOL_INTERFACE = extern "C" fn(
  Handle: EFI_HANDLE,  // IN
  Protocol: &EFI_GUID, // IN
  Interface: VOID_PTR, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.ReinstallProtocolInterface()
pub type EFI_REINSTALL_PROTOCOL_INTERFACE = extern "C" fn(
  Handle: EFI_HANDLE,     // IN
  Protocol: &EFI_GUID,    // IN
  OldInterface: VOID_PTR, // IN
  NewInterface: VOID_PTR, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.RegisterProtocolNotify()
pub type EFI_REGISTER_PROTOCOL_NOTIFY = extern "C" fn(
  Protocol: &EFI_GUID,         // IN
  Event: EFI_EVENT,            // IN
  Registration: &mut VOID_PTR, // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.LocateHandle()
pub type EFI_LOCATE_HANDLE = extern "C" fn(
  SearchType: EFI_LOCATE_SEARCH_TYPE, // IN
  Protocol: Option<&EFI_GUID>,        // IN
  SearchKey: Option<VOID_PTR>,        // IN
  BufferSize: &mut usize,             // IN OUT
  Buffer: EFI_HANDLE,                 // OUT
) -> EFI_STATUS;

#[repr(C)]
pub enum EFI_LOCATE_SEARCH_TYPE {
  AllHandles,
  ByRegisterNotify,
  ByProtocol,
}

// EFI_BOOT_SERVICES.HandleProtocol()
pub type EFI_HANDLE_PROTOCOL = extern "C" fn(
  Handle: EFI_HANDLE,       // IN
  Protocol: &EFI_GUID,      // IN
  Interface: &mut VOID_PTR, // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.LocateDevicePath()
pub type EFI_LOCATE_DEVICE_PATH = extern "C" fn(
  Protocol: &EFI_GUID,                            // IN
  DevicePath: &mut &mut EFI_DEVICE_PATH_PROTOCOL, // IN OUT
  Device: &mut EFI_HANDLE,                        // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
  Type: u8,
  SubType: u8,
  Length: [u8; 2],
}

// EFI_BOOT_SERVICES.OpenProtocol()
pub type EFI_OPEN_PROTOCOL = extern "C" fn(
  Handle: EFI_HANDLE,               // IN
  Protocol: &EFI_GUID,              // IN
  Interface: Option<&mut VOID_PTR>, // OUT
  AgentHandle: EFI_HANDLE,          // IN
  ControllerHandle: EFI_HANDLE,     // IN
  Attributes: u32,                  // IN
) -> EFI_STATUS;

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;
pub const EFI_OPEN_PROTOCOL_GET_PROTOCOL: u32 = 0x00000002;
pub const EFI_OPEN_PROTOCOL_TEST_PROTOCOL: u32 = 0x00000004;
pub const EFI_OPEN_PROTOCOL_BY_CHILD_CONTROLLER: u32 = 0x00000008;
pub const EFI_OPEN_PROTOCOL_BY_DRIVER: u32 = 0x00000010;
pub const EFI_OPEN_PROTOCOL_EXCLUSIVE: u32 = 0x00000020;

// EFI_BOOT_SERVICES.CloseProtocol()
pub type EFI_CLOSE_PROTOCOL = extern "C" fn(
  Handle: EFI_HANDLE,           // IN
  Protocol: &EFI_GUID,          // IN
  AgentHandle: EFI_HANDLE,      // IN
  ControllerHandle: EFI_HANDLE, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.OpenProtocolInformation()
pub type EFI_OPEN_PROTOCOL_INFORMATION = extern "C" fn(
  Handle: EFI_HANDLE,                                         // IN
  Protocol: &EFI_GUID,                                        // IN
  EntryBuffer: &mut &mut EFI_OPEN_PROTOCOL_INFORMATION_ENTRY, // OUT
  EntryCount: &mut usize,                                     // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
  AgentHandle: EFI_HANDLE,
  ControllerHandle: EFI_HANDLE,
  Attributes: u32,
  OpenCount: u32,
}

// EFI_BOOT_SERVICES.ConnectController()
pub type EFI_CONNECT_CONTROLLER = extern "C" fn(
  ControllerHandle: EFI_HANDLE,                               // IN
  DriverImageHandle: Option<&EFI_HANDLE>,                     // IN
  RemainingDevicePath: Option<&mut EFI_DEVICE_PATH_PROTOCOL>, // IN
  Recursive: bool,                                            // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.DisconnectController()
pub type EFI_DISCONNECT_CONTROLLER = extern "C" fn(
  ControllerHandle: EFI_HANDLE,          // IN
  DriverImageHandle: Option<EFI_HANDLE>, // IN
  ChildHandle: Option<EFI_HANDLE>,       // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.ProtocolsPerHandle()
pub type EFI_PROTOCOLS_PER_HANDLE = extern "C" fn(
  Handle: EFI_HANDLE,                      // IN
  ProtocolBuffer: &mut &mut &mut EFI_GUID, // OUT
  ProtocolBufferCount: &mut usize,         // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.LocateHandleBuffer()
pub type EFI_LOCATE_HANDLE_BUFFER = extern "C" fn(
  SearchType: EFI_LOCATE_SEARCH_TYPE, // IN
  Protocol: Option<&EFI_GUID>,        // IN
  SearchKey: Option<VOID_PTR>,        // IN
  NoHandles: &mut usize,              // IN OUT
  Buffer: &mut &mut EFI_HANDLE,       // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.LocateProtocol()
pub type EFI_LOCATE_PROTOCOL = extern "C" fn(
  Protocol: &EFI_GUID,            // IN
  Registration: Option<VOID_PTR>, // IN
  Interface: &mut VOID_PTR,       // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.InstallMultipleProtocolInterfaces()
pub type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = extern "C" fn() -> EFI_STATUS;

// EFI_BOOT_SERVICES.UninstallMultipleProtocolInterfaces()
pub type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = extern "C" fn() -> EFI_STATUS;

// EFI_BOOT_SERVICES.LoadImage()
pub type EFI_IMAGE_LOAD = extern "C" fn(
  BootPolicy: bool,                      // IN
  ParentImageHandle: EFI_HANDLE,         // IN
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
  SourceBuffer: Option<VOID_PTR>,        // IN
  SourceSize: usize,                     // IN
  ImageHandle: &mut EFI_HANDLE,          // OUT
) -> EFI_STATUS;

pub const EFI_HII_PACKAGE_LIST_PROTOCOL_GUID: EFI_GUID = 0x6a1ee763d47a43b4aabeef1de2ab56fc;

// EFI_BOOT_SERVICES.StartImage()
pub type EFI_IMAGE_START = extern "C" fn(
  ImageHandle: EFI_HANDLE,         // IN
  ExitDataSize: &mut usize,        // OUT
  ExitData: Option<&mut &mut u16>, // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.UnloadImage()
pub type EFI_IMAGE_UNLOAD = extern "C" fn(
  ImageHandle: EFI_HANDLE, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.Exit()
pub type EFI_EXIT = extern "C" fn(
  ImageHandle: EFI_HANDLE,      // IN
  ExitStatus: EFI_STATUS,       // IN
  ExitDataSize: usize,          // IN
  ExitData: Option<*const u16>, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.ExitBootServices()
pub type EFI_EXIT_BOOT_SERVICES = extern "C" fn(
  ImageHandle: EFI_HANDLE, // IN
  MapKey: usize,           // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.SetWatchdogTimer()
pub type EFI_SET_WATCHDOG_TIMER = extern "C" fn(
  Timeout: usize,                   // IN
  WatchdogCode: u64,                // IN
  DataSize: usize,                  // IN
  WatchdogData: Option<*const u16>, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.Stall()
pub type EFI_STALL = extern "C" fn(
  Microseconds: usize, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.CopyMem()
pub type EFI_COPY_MEM = extern "C" fn(
  Destination: VOID_PTR, // IN
  Source: VOID_PTR,      // IN
  Length: usize,         // IN
);

// EFI_BOOT_SERVICES.SetMem()
pub type EFI_SET_MEM = extern "C" fn(
  Buffer: VOID_PTR, // IN
  Size: usize,      // IN
  Value: u8,        // IN
);

// EFI_BOOT_SERVICES.GetNextMonotonicCount()
pub type EFI_GET_NEXT_MONOTONIC_COUNT = extern "C" fn(
  Count: &mut u64, // OUT
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.InstallConfigurationTable()
pub type EFI_INSTALL_CONFIGURATION_TABLE = extern "C" fn(
  Guid: &EFI_GUID, // IN
  Table: VOID_PTR, // IN
) -> EFI_STATUS;

// EFI_BOOT_SERVICES.CalculateCrc32()
pub type EFI_CALCULATE_CRC32 = extern "C" fn(
  Data: VOID_PTR,  // IN
  DataSize: usize, // IN
  Crc32: &mut u32, // OUT
) -> EFI_STATUS;
