use crate::protocols::device_path_protocols::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::*;

// EFI_DRIVER_BINDING_PROTOCOL
#[repr(C)]
pub struct EFI_DRIVER_BINDING_PROTOCOL {
  pub Supported: EFI_DRIVER_BINDING_PROTOCOL_SUPPORTED,
  pub Start: EFI_DRIVER_BINDING_PROTOCOL_START,
  pub Stop: EFI_DRIVER_BINDING_PROTOCOL_STOP,
  pub Version: u32,
  pub ImageHandle: EFI_HANDLE,
  pub DriverBindingHandle: EFI_HANDLE,
}

// EFI_DRIVER_BINDING_PROTOCOL.Supported()
pub type EFI_DRIVER_BINDING_PROTOCOL_SUPPORTED = extern "C" fn(
  This: &EFI_DRIVER_BINDING_PROTOCOL,                     // IN
  ControllerHandle: EFI_HANDLE,                           // IN
  RemainingDevicePath: Option<&EFI_DEVICE_PATH_PROTOCOL>, // IN
) -> EFI_STATUS;

// EFI_DRIVER_BINDING_PROTOCOL.Start()
pub type EFI_DRIVER_BINDING_PROTOCOL_START = extern "C" fn(
  This: &EFI_DRIVER_BINDING_PROTOCOL,                     // IN
  ControllerHandle: EFI_HANDLE,                           // IN
  RemainingDevicePath: Option<&EFI_DEVICE_PATH_PROTOCOL>, // IN
) -> EFI_STATUS;

// EFI_DRIVER_BINDING_PROTOCOL.Stop()
pub type EFI_DRIVER_BINDING_PROTOCOL_STOP = extern "C" fn(
  This: &EFI_DRIVER_BINDING_PROTOCOL,     // IN
  ControllerHandle: EFI_HANDLE,           // IN
  NumberOfChildren: usize,                // IN
  ChildHandleBuffer: Option<&EFI_HANDLE>, // IN
) -> EFI_STATUS;

// EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL
#[repr(C)]
pub struct EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL {
  pub GetDriver: EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER,
  pub GetDriverPath: EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER_PATH,
  pub DriverLoaded: EFI_PLATFORM_DRIVER_OVERRIDE_DRIVER_LOADED,
}

// EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL.GetDriver()
pub type EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER = extern "C" fn(
  This: &EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL, // IN
  ControllerHandle: EFI_HANDLE,                 // IN
  DriverImageHandle: &mut EFI_HANDLE,           // IN OUT
) -> EFI_STATUS;

// EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL.GetDriverPath()
pub type EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER_PATH = extern "C" fn(
  This: &EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL,    // IN
  ControllerHandle: EFI_HANDLE,                    // IN
  DriverImagePath: &mut &EFI_DEVICE_PATH_PROTOCOL, // IN OUT
) -> EFI_STATUS;

// EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL.DriverLoaded()
pub type EFI_PLATFORM_DRIVER_OVERRIDE_DRIVER_LOADED = extern "C" fn(
  This: &EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL, // IN
  ControllerHandle: EFI_HANDLE,                 // IN
  DriverImagePath: &EFI_DEVICE_PATH_PROTOCOL,   // IN
  DriverImageHandle: EFI_HANDLE,                // IN
) -> EFI_STATUS;

// EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL
#[repr(C)]
pub struct EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL {
  pub GetDriver: EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_GET_DRIVER,
}

// EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL.GetDriver()
pub type EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_GET_DRIVER = extern "C" fn(
  This: &EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL, // IN
  DriverImageHandle: &mut EFI_HANDLE,               // IN OUT
) -> EFI_STATUS;

// EFI_DRIVER_DIAGNOSTICS2_PROTOCOL
#[repr(C)]
pub struct EFI_DRIVER_DIAGNOSTICS2_PROTOCOL<'a> {
  pub RunDiagnostics: EFI_DRIVER_DIAGNOSTICS2_RUN_DIAGNOSTICS,
  pub SupportedLanguages: &'a [u8],
}

// EFI_DRIVER_DIAGNOSTICS2_PROTOCOL.RunDiagnostics()
pub type EFI_DRIVER_DIAGNOSTICS2_RUN_DIAGNOSTICS = extern "C" fn(
  This: &EFI_DRIVER_DIAGNOSTICS2_PROTOCOL,    // IN
  ControllerHandle: EFI_HANDLE,               // IN
  ChildHandle: Option<EFI_HANDLE>,            // IN
  DiagnosticType: EFI_DRIVER_DIAGNOSTIC_TYPE, // IN
  Language: &[u8],                            // IN
  ErrorType: &mut &EFI_GUID,                  // OUT
  BufferSize: &usize,                         // OUT
  Buffer: &mut &[u16],                        // OUT
) -> EFI_STATUS;

// EFI_DRIVER_DIAGNOSTIC_TYPE
#[repr(C)]
pub enum EFI_DRIVER_DIAGNOSTIC_TYPE {
  EfiDriverDiagnosticTypeStandard = 0,
  EfiDriverDiagnosticTypeExtended = 1,
  EfiDriverDiagnosticTypeManufacturing = 2,
  EfiDriverDiagnosticTypeCancel = 3,
  EfiDriverDiagnosticTypeMaximum,
}

// EFI_COMPONENT_NAME2_PROTOCOL
#[repr(C)]
pub struct EFI_COMPONENT_NAME2_PROTOCOL<'a> {
  pub GetDriverName: EFI_COMPONENT_NAME_GET_DRIVER_NAME,
  pub GetControllerName: EFI_COMPONENT_NAME_GET_CONTROLLER_NAME,
  pub SupportedLanguages: &'a [u8],
}

// EFI_COMPONENT_NAME2_PROTOCOL.GetDriverName()
pub type EFI_COMPONENT_NAME_GET_DRIVER_NAME = extern "C" fn(
  This: &EFI_COMPONENT_NAME2_PROTOCOL, // IN
  Language: &[u8],                     // IN
  DriverName: &mut &[u16],             // OUT
) -> EFI_STATUS;

// EFI_COMPONENT_NAME2_PROTOCOL.GetControllerName()
pub type EFI_COMPONENT_NAME_GET_CONTROLLER_NAME = extern "C" fn(
  This: &EFI_COMPONENT_NAME2_PROTOCOL, // IN
  ControllerHandle: EFI_HANDLE,        // IN
  ChildHandle: Option<EFI_HANDLE>,     // IN
  Language: &[u8],                     // IN
  ControllerName: &mut &[u16],         // OUT
) -> EFI_STATUS;

// EFI_SERVICE_BINDING_PROTOCOL
#[repr(C)]
pub struct EFI_SERVICE_BINDING_PROTOCOL {
  pub CreateChild: EFI_SERVICE_BINDING_CREATE_CHILD,
  pub DestroyChild: EFI_SERVICE_BINDING_DESTROY_CHILD,
}

// EFI_SERVICE_BINDING_PROTOCOL.CreateChild()
pub type EFI_SERVICE_BINDING_CREATE_CHILD = extern "C" fn(
  This: &EFI_SERVICE_BINDING_PROTOCOL, // IN
  ChildHandle: &mut EFI_HANDLE,        // IN OUT
) -> EFI_STATUS;

// EFI_SERVICE_BINDING_PROTOCOL.DestroyChild()
pub type EFI_SERVICE_BINDING_DESTROY_CHILD = extern "C" fn(
  This: &EFI_SERVICE_BINDING_PROTOCOL, // IN
  ChildHandle: EFI_HANDLE,             // IN
) -> EFI_STATUS;

// EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL
#[repr(C)]
pub struct EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL {
  pub Query: EFI_PLATFORM_TO_DRIVER_CONFIGURATION_QUERY,
  pub Response: EFI_PLATFORM_TO_DRIVER_CONFIGURATION_RESPONSE,
}

// EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL.Query()
pub type EFI_PLATFORM_TO_DRIVER_CONFIGURATION_QUERY = extern "C" fn(
  This: &EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL, // IN
  ControllerHandle: EFI_HANDLE,                         // IN
  ChildHandle: Option<EFI_HANDLE>,                      // IN
  Instance: &usize,                                     // IN
  ParameterTypeGuid: &mut &EFI_GUID,                    // OUT
  ParameterBlock: &mut *mut u8,                         // OUT
  ParameterBlockSize: &mut usize,                       // OUT
) -> EFI_STATUS;

// EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL.Response()
pub type EFI_PLATFORM_TO_DRIVER_CONFIGURATION_RESPONSE = extern "C" fn(
  This: &EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL, // IN
  ControllerHandle: EFI_HANDLE,                         // IN
  ChildHandle: Option<EFI_HANDLE>,                      // IN
  Instance: &usize,                                     // IN
  ParameterTypeGuid: &EFI_GUID,                         // IN
  ParameterBlock: *mut u8,                              // IN
  ParameterBlockSize: usize,                            // IN
  ConfigurationAction: EFI_PLATFORM_CONFIGURATION_ACTION, // IN
) -> EFI_STATUS;

#[repr(C)]
pub enum EFI_PLATFORM_CONFIGURATION_ACTION {
  EfiPlatformConfigurationActionNone = 0,
  EfiPlatformConfigurationActionStopController = 1,
  EfiPlatformConfigurationActionRestartController = 2,
  EfiPlatformConfigurationActionRestartPlatform = 3,
  EfiPlatformConfigurationActionNvramFailed = 4,
  EfiPlatformConfigurationActionUnsupportedGuid = 5,
  EfiPlatformConfigurationActionMaximum,
}

// DMTF SM CLP ParameterTypeGuid
#[repr(C)]
pub struct EFI_CONFIGURE_CLP_PARAMETER_BLK<'a> {
  pub CLPCommand: &'a [u8],
  pub CLPCommandLength: u32,
  pub CLPReturnString: &'a [u8],
  pub CLPReturnStringLength: u32,
  pub CLPCmdStatus: u8,
  pub CLPErrorValue: u8,
  pub CLPMsgCode: u16,
}

// EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL
#[repr(C)]
pub struct EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL {
  pub Length: u32,
  pub FirmwareVersion: u32,
}

// EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL
#[repr(C)]
pub struct EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL {
  GetVersion: EFI_DRIVER_FAMILY_OVERRIDE_GET_VERSION,
}

// EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL.GetVersion ()
pub type EFI_DRIVER_FAMILY_OVERRIDE_GET_VERSION = extern "C" fn(
  This: &EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL, // IN
) -> u32;

// EFI_DRIVER_HEALTH_PROTOCOL
#[repr(C)]
pub struct EFI_DRIVER_HEALTH_PROTOCOL {
  pub GetHealthStatus: EFI_DRIVER_HEALTH_GET_HEALTH_STATUS,
  pub Repair: EFI_DRIVER_HEALTH_REPAIR,
}

// EFI_DRIVER_HEALTH_PROTOCOL.GetHealthStatus()
pub type EFI_DRIVER_HEALTH_GET_HEALTH_STATUS = extern "C" fn(
  This: &EFI_DRIVER_HEALTH_PROTOCOL,                          // IN
  ControllerHandle: Option<EFI_HANDLE>,                       // IN
  ChildHandle: Option<EFI_HANDLE>,                            // IN
  HealthStatus: &mut EFI_DRIVER_HEALTH_STATUS,                // OUT
  MessageList: Option<&mut &[EFI_DRIVER_HEALTH_HII_MESSAGE]>, // OUT
  FormHiiHandle: Option<&mut EFI_HII_HANDLE>,                 // OUT
) -> EFI_STATUS;

// EFI_DRIVER_HEALTH_STATUS
#[repr(C)]
pub enum EFI_DRIVER_HEALTH_STATUS {
  EfiDriverHealthStatusHealthy,
  EfiDriverHealthStatusRepairRequired,
  EfiDriverHealthStatusConfigurationRequired,
  EfiDriverHealthStatusFailed,
  EfiDriverHealthStatusReconnectRequired,
  EfiDriverHealthStatusRebootRequired,
}

pub struct EFI_HII_HANDLE;
pub struct EFI_STRING_ID;
// EFI_DRIVER_HEALTH_HII_MESSAGE
#[repr(C)]
pub struct EFI_DRIVER_HEALTH_HII_MESSAGE {
  pub HiiHandle: EFI_HII_HANDLE,
  pub StringId: EFI_STRING_ID,
  pub MessageCode: u64,
}

// EFI_DRIVER_HEALTH_PROTOCOL.Repair ()
pub type EFI_DRIVER_HEALTH_REPAIR = extern "C" fn(
  This: &EFI_DRIVER_HEALTH_PROTOCOL,                     // IN
  ControllerHandle: EFI_HANDLE,                          // IN
  ChildHandle: Option<EFI_HANDLE>,                       // IN
  RepairNotify: Option<EFI_DRIVER_HEALTH_REPAIR_NOTIFY>, // IN
) -> EFI_STATUS;

// EFI_DRIVER_HEALTH_REPAIR_NOTIFY
pub type EFI_DRIVER_HEALTH_REPAIR_NOTIFY = extern "C" fn(
  Value: usize, // IN
  Limit: usize, // IN
) -> EFI_STATUS;

// EFI_ADAPTER_INFORMATION_PROTOCOL
#[repr(C)]
pub struct EFI_ADAPTER_INFORMATION_PROTOCOL {
  pub GetInformation: EFI_ADAPTER_INFO_GET_INFO,
  pub SetInformation: EFI_ADAPTER_INFO_SET_INFO,
  pub GetSupportedTypes: EFI_ADAPTER_INFO_GET_SUPPORTED_TYPES,
}

// EFI_ADAPTER_INFORMATION_PROTOCOL.EFI_ADAPTER_INFO_GET_INFO()
pub type EFI_ADAPTER_INFO_GET_INFO = extern "C" fn(
  This: &EFI_ADAPTER_INFORMATION_PROTOCOL, // IN
  InformationType: &EFI_GUID,              // IN
  InformationBlock: &mut *const u8,        // OUT
  InformationBlockSize: &mut usize,        // OUT
) -> EFI_STATUS;

// EFI_ADAPTER_INFORMATION_PROTOCOL.EFI_ADAPTER_INFO_SET_INFO()
pub type EFI_ADAPTER_INFO_SET_INFO = extern "C" fn(
  This: &EFI_ADAPTER_INFORMATION_PROTOCOL, // IN
  InformationType: &EFI_GUID,              // IN
  InformationBlock: *const u8,             // IN
  InformationBlockSize: usize,             // IN
) -> EFI_STATUS;

// EFI_ADAPTER_INFORMATION_PROTOCOL.EFI_ADAPTER_INFO_GET_SUPPORTED_TYPES()
pub type EFI_ADAPTER_INFO_GET_SUPPORTED_TYPES = extern "C" fn(
  This: &EFI_ADAPTER_INFORMATION_PROTOCOL, // IN
  InfoTypesBuffer: &mut &[EFI_GUID],       // OUT
  InfoTypesBufferCount: &usize,            // OUT
) -> EFI_STATUS;

// Network Media State
#[repr(C)]
pub struct EFI_ADAPTER_INFO_MEDIA_STATE {
  pub MediaState: EFI_STATUS,
}

// Network Boot
#[repr(C)]
pub struct EFI_ADAPTER_INFO_NETWORK_BOOT {
  pub iSsciIpv4BootCapablity: bool,
  pub iScsiIpv6BootCapablity: bool,
  pub FCoeBootCapablity: bool,
  pub OffloadCapability: bool,
  pub iScsiMpioCapabilit: bool,
  pub iScsiIpv4Boot: bool,
  pub iScsiIpv6Boot: bool,
  pub FCoeBoot: bool,
}

// SAN MAC Address
#[repr(C)]
pub struct EFI_ADAPTER_INFO_SAN_MAC_ADDRESS {
  pub SanMacAddress: EFI_MAC_ADDRESS,
}

// IPV6 Support from UNDI
#[repr(C)]
pub struct EFI_ADAPTER_INFO_UNDI_IPV6_SUPPORT {
  Ipv6Support: bool,
}

//  Network Media Type
#[repr(C)]
pub struct EFI_ADAPTER_INFO_MEDIA_TYPE {
  pub MediaType: u8,
}
