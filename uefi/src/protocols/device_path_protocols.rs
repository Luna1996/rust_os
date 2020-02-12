use crate::*;

// EFI_DEVICE_PATH_PROTOCOL
pub const EFI_DEVICE_PATH_PROTOCOL_GUID: EFI_GUID = 0x09576e916d3f11d28e3900a0c969723b;
#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
  pub Type: u8,
  pub SubType: u8,
  pub Length: [u8; 2],
}

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL
pub const EFI_DEVICE_PATH_UTILITIES_PROTOCOL_GUID: EFI_GUID = 0x0379BE4ED706437dB037EDB82FB772A4;

#[repr(C)]
pub struct EFI_DEVICE_PATH_UTILITIES_PROTOCOL<'a> {
  pub GetDevicePathSize: EFI_DEVICE_PATH_UTILS_GET_DEVICE_PATH_SIZE,
  pub DuplicateDevicePath: EFI_DEVICE_PATH_UTILS_DUP_DEVICE_PATH,
  pub AppendDevicePath: EFI_DEVICE_PATH_UTILS_APPEND_PATH<'a>,
  pub AppendDeviceNode: EFI_DEVICE_PATH_UTILS_APPEND_NODE<'a>,
  pub AppendDevicePathInstance: EFI_DEVICE_PATH_UTILS_APPEND_INSTANCE<'a>,
  pub GetNextDevicePathInstance: EFI_DEVICE_PATH_UTILS_GET_NEXT_INSTANCE<'a>,
  pub IsDevicePathMultiInstance: EFI_DEVICE_PATH_UTILS_IS_MULTI_INSTANCE,
  pub CreateDeviceNode: EFI_DEVICE_PATH_UTILS_CREATE_NODE<'a>,
}

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.GetDevicePathSize()
pub type EFI_DEVICE_PATH_UTILS_GET_DEVICE_PATH_SIZE = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> usize;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.DuplicateDevicePath()
pub type EFI_DEVICE_PATH_UTILS_DUP_DEVICE_PATH = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> &EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.AppendDevicePath()
pub type EFI_DEVICE_PATH_UTILS_APPEND_PATH<'a> = extern "C" fn(
  Src1: &EFI_DEVICE_PATH_PROTOCOL, // IN
  Src2: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.AppendDeviceNode()
pub type EFI_DEVICE_PATH_UTILS_APPEND_NODE<'a> = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
  DeviceNode: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.AppendDevicePathInstance()
pub type EFI_DEVICE_PATH_UTILS_APPEND_INSTANCE<'a> = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL,         // IN
  DevicePathInstance: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.GetNextDevicePathInstance()
pub type EFI_DEVICE_PATH_UTILS_GET_NEXT_INSTANCE<'a> =
  extern "C" fn(
    DevicePathInstance: &mut &EFI_DEVICE_PATH_PROTOCOL, // IN OUT
    DevicePathInstanceSize: Option<&mut usize>,         // OUT
  ) -> &'a EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.CreateDeviceNode()
pub type EFI_DEVICE_PATH_UTILS_CREATE_NODE<'a> = extern "C" fn(
  NodeType: u8,    // IN
  NodeSubType: u8, // IN
  NodeLength: u16, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;

// EFI_DEVICE_PATH_UTILITIES_PROTOCOL.IsDevicePathMultiInstance()
pub type EFI_DEVICE_PATH_UTILS_IS_MULTI_INSTANCE = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
) -> bool;

// EFI_DEVICE_PATH_TO_TEXT_PROTOCOL
pub const EFI_DEVICE_PATH_TO_TEXT_PROTOCOL_GUID: EFI_GUID = 0x8b843e208132485290cc551a4e4a7f1c;

#[repr(C)]
pub struct _EFI_DEVICE_PATH_TO_TEXT_PROTOCOL {
  pub ConvertDeviceNodeToText: EFI_DEVICE_PATH_TO_TEXT_NODE,
  pub ConvertDevicePathToText: EFI_DEVICE_PATH_TO_TEXT_PATH,
}

pub type EFI_DEVICE_PATH_TO_TEXT_NODE = extern "C" fn(
  DeviceNode: &EFI_DEVICE_PATH_PROTOCOL, // IN
  DisplayOnly: bool,                     // IN
  AllowShortcuts: bool,                  // IN
) -> *const u16;

pub type EFI_DEVICE_PATH_TO_TEXT_PATH = extern "C" fn(
  DevicePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
  DisplayOnly: bool,                     // IN
  AllowShortcuts: bool,                  // IN
) -> *const u16;

// EFI_DEVICE_PATH_FROM_TEXT_PROTOCOL
pub const EFI_DEVICE_PATH_FROM_TEXT_PROTOCOL_GUID: EFI_GUID = 0x5c99a21c70f4ad28a5f35df3343f51e;

#[repr(C)]
pub struct _EFI_DEVICE_PATH_FROM_TEXT_PROTOCOL<'a> {
  pub ConvertTextToDevicNode: EFI_DEVICE_PATH_FROM_TEXT_NODE<'a>,
  pub ConvertTextToDevicPath: EFI_DEVICE_PATH_FROM_TEXT_PATH<'a>,
}

pub type EFI_DEVICE_PATH_FROM_TEXT_NODE<'a> = extern "C" fn(
  TextDeviceNode: *const u16, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;

pub type EFI_DEVICE_PATH_FROM_TEXT_PATH<'a> = extern "C" fn(
  TextDevicePath: *const u16, // IN
) -> &'a EFI_DEVICE_PATH_PROTOCOL;
