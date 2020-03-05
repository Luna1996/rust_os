#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::types::*;

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
  pub Reset: EFI_INPUT_RESET_EX,
  pub ReadKeyStrokeEx: EFI_INPUT_READ_KEY_EX,
  pub WaitForKeyEx: EFI_EVENT,
  pub SetState: EFI_SET_STATE,
  pub RegisterKeyNotify: EFI_REGISTER_KEYSTROKE_NOTIFY,
  pub UnregisterKeyNotify: EFI_UNREGISTER_KEYSTROKE_NOTIFY,
}

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.Reset()
pub type EFI_INPUT_RESET_EX = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL, // IN
  ExtendedVerification: bool,               // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.ReadKeyStrokeEx()
pub type EFI_INPUT_READ_KEY_EX = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL, // IN
  KeyData: &mut EFI_KEY_DATA,               // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_KEY_DATA {
  pub Key: EFI_INPUT_KEY,
  pub KeyState: EFI_KEY_STATE,
}

#[repr(C)]
pub struct EFI_KEY_STATE {
  pub KeyShiftState: u32,
  pub KeyToggleState: u8,
}

pub const EFI_SHIFT_STATE_VALID: u32 = 0x80000000;
pub const EFI_RIGHT_SHIFT_PRESSED: u32 = 0x00000001;
pub const EFI_LEFT_SHIFT_PRESSED: u32 = 0x00000002;
pub const EFI_RIGHT_CONTROL_PRESSED: u32 = 0x00000004;
pub const EFI_LEFT_CONTROL_PRESSED: u32 = 0x00000008;
pub const EFI_RIGHT_ALT_PRESSED: u32 = 0x00000010;
pub const EFI_LEFT_ALT_PRESSED: u32 = 0x00000020;
pub const EFI_RIGHT_LOGO_PRESSED: u32 = 0x00000040;
pub const EFI_LEFT_LOGO_PRESSED: u32 = 0x00000080;
pub const EFI_MENU_KEY_PRESSED: u32 = 0x00000100;
pub const EFI_SYS_REQ_PRESSED: u32 = 0x00000200;

pub const EFI_TOGGLE_STATE_VALID: u8 = 0x80;
pub const EFI_KEY_STATE_EXPOSED: u8 = 0x40;
pub const EFI_SCROLL_LOCK_ACTIVE: u8 = 0x01;
pub const EFI_NUM_LOCK_ACTIVE: u8 = 0x02;
pub const EFI_CAPS_LOCK_ACTIVE: u8 = 0x04;

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.SetState()
pub type EFI_SET_STATE = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL, // IN
  KeyToggleState: &u8,                      // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.RegisterKeyNotify()
pub type EFI_REGISTER_KEYSTROKE_NOTIFY = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,         // IN
  KeyData: &EFI_KEY_DATA,                           // IN
  KeyNotificationFunction: EFI_KEY_NOTIFY_FUNCTION, // IN
  NotifyHandle: &mut VOID_PTR,                      // OUT
) -> EFI_STATUS;

pub type EFI_KEY_NOTIFY_FUNCTION = extern "C" fn(
  KeyData: &EFI_KEY_DATA, // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.UnregisterKeyNotify()
pub type EFI_UNREGISTER_KEYSTROKE_NOTIFY = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL, // IN
  NotificationHandle: VOID_PTR,             // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_INPUT_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
  pub Reset: EFI_INPUT_RESET,
  pub ReadKeyStroke: EFI_INPUT_READ_KEY,
  pub WaitForKey: EFI_EVENT,
}

// EFI_SIMPLE_TEXT_INPUT_PROTOCOL.Reset()
pub type EFI_INPUT_RESET = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_PROTOCOL, // IN
  ExtendedVerification: bool,            // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_INPUT_PROTOCOL.ReadKeyStroke()
pub type EFI_INPUT_READ_KEY = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_INPUT_PROTOCOL, // IN
  Key: &mut EFI_INPUT_KEY,               // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_INPUT_KEY {
  pub ScanCode: u16,
  pub UnicodeChar: u16,
}

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL<'a> {
  pub Reset: EFI_TEXT_RESET,
  pub OutputString: EFI_TEXT_STRING,
  pub TestString: EFI_TEXT_TEST_STRING,
  pub QueryMode: EFI_TEXT_QUERY_MODE,
  pub SetMode: EFI_TEXT_SET_MODE,
  pub SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
  pub ClearScreen: EFI_TEXT_CLEAR_SCREEN,
  pub SetCursorPosition: EFI_TEXT_SET_CURSOR_POSITION,
  pub EnableCursor: EFI_TEXT_ENABLE_CURSOR,
  pub Mode: &'a mut SIMPLE_TEXT_OUTPUT_MODE,
}

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
  pub MaxMode: i32,
  // current settings
  pub Mode: i32,
  pub Attribute: i32,
  pub CursorColumn: i32,
  pub CursorRow: i32,
  pub CursorVisible: bool,
}

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.Reset()
pub type EFI_TEXT_RESET = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  ExtendedVerification: bool,             // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()
pub type EFI_TEXT_STRING = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  Str: *const u16,                        // IN
) -> EFI_STATUS;

//*******************************************************
// UNICODE DRAWING CHARACTERS
//*******************************************************
pub const BOXDRAW_HORIZONTAL: u16 = 0x2500;
pub const BOXDRAW_VERTICAL: u16 = 0x2502;
pub const BOXDRAW_DOWN_RIGHT: u16 = 0x250c;
pub const BOXDRAW_DOWN_LEFT: u16 = 0x2510;
pub const BOXDRAW_UP_RIGHT: u16 = 0x2514;
pub const BOXDRAW_UP_LEFT: u16 = 0x2518;
pub const BOXDRAW_VERTICAL_RIGHT: u16 = 0x251c;
pub const BOXDRAW_VERTICAL_LEFT: u16 = 0x2524;
pub const BOXDRAW_DOWN_HORIZONTAL: u16 = 0x252c;
pub const BOXDRAW_UP_HORIZONTAL: u16 = 0x2534;
pub const BOXDRAW_VERTICAL_HORIZONTAL: u16 = 0x253c;
pub const BOXDRAW_DOUBLE_HORIZONTAL: u16 = 0x2550;
pub const BOXDRAW_DOUBLE_VERTICAL: u16 = 0x2551;
pub const BOXDRAW_DOWN_RIGHT_DOUBLE: u16 = 0x2552;
pub const BOXDRAW_DOWN_DOUBLE_RIGHT: u16 = 0x2553;
pub const BOXDRAW_DOUBLE_DOWN_RIGHT: u16 = 0x2554;
pub const BOXDRAW_DOWN_LEFT_DOUBLE: u16 = 0x2555;
pub const BOXDRAW_DOWN_DOUBLE_LEFT: u16 = 0x2556;
pub const BOXDRAW_DOUBLE_DOWN_LEFT: u16 = 0x2557;
pub const BOXDRAW_UP_RIGHT_DOUBLE: u16 = 0x2558;
pub const BOXDRAW_UP_DOUBLE_RIGHT: u16 = 0x2559;
pub const BOXDRAW_DOUBLE_UP_RIGHT: u16 = 0x255a;
pub const BOXDRAW_UP_LEFT_DOUBLE: u16 = 0x255b;
pub const BOXDRAW_UP_DOUBLE_LEFT: u16 = 0x255c;
pub const BOXDRAW_DOUBLE_UP_LEFT: u16 = 0x255d;
pub const BOXDRAW_VERTICAL_RIGHT_DOUBLE: u16 = 0x255e;
pub const BOXDRAW_VERTICAL_DOUBLE_RIGHT: u16 = 0x255f;
pub const BOXDRAW_DOUBLE_VERTICAL_RIGHT: u16 = 0x2560;
pub const BOXDRAW_VERTICAL_LEFT_DOUBLE: u16 = 0x2561;
pub const BOXDRAW_VERTICAL_DOUBLE_LEFT: u16 = 0x2562;
pub const BOXDRAW_DOUBLE_VERTICAL_LEFT: u16 = 0x2563;
pub const BOXDRAW_DOWN_HORIZONTAL_DOUBLE: u16 = 0x2564;
pub const BOXDRAW_DOWN_DOUBLE_HORIZONTAL: u16 = 0x2565;
pub const BOXDRAW_DOUBLE_DOWN_HORIZONTAL: u16 = 0x2566;
pub const BOXDRAW_UP_HORIZONTAL_DOUBLE: u16 = 0x2567;
pub const BOXDRAW_UP_DOUBLE_HORIZONTAL: u16 = 0x2568;
pub const BOXDRAW_DOUBLE_UP_HORIZONTAL: u16 = 0x2569;
pub const BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE: u16 = 0x256a;
pub const BOXDRAW_VERTICAL_DOUBLE_HORIZONTAL: u16 = 0x256b;
pub const BOXDRAW_DOUBLE_VERTICAL_HORIZONTAL: u16 = 0x256c;
//*******************************************************
// EFI Required Block Elements Code Chart
//*******************************************************
pub const BLOCKELEMENT_FULL_BLOCK: u16 = 0x2588;
pub const BLOCKELEMENT_LIGHT_SHADE: u16 = 0x2591;
//*******************************************************
// EFI Required Geometric Shapes Code Chart
//*******************************************************
pub const GEOMETRICSHAPE_UP_TRIANGLE: u16 = 0x25b2;
pub const GEOMETRICSHAPE_RIGHT_TRIANGLE: u16 = 0x25ba;
pub const GEOMETRICSHAPE_DOWN_TRIANGLE: u16 = 0x25bc;
pub const GEOMETRICSHAPE_LEFT_TRIANGLE: u16 = 0x25c4;
//*******************************************************
// EFI Required Arrow shapes
//*******************************************************
pub const ARROW_UP: u16 = 0x2191;
pub const ARROW_DOWN: u16 = 0x2193;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.TestString()
pub type EFI_TEXT_TEST_STRING = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  Str: &u16,                              // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.QueryMode()
pub type EFI_TEXT_QUERY_MODE = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  ModeNumber: usize,                      // IN
  Columns: &mut usize,                    // OUT
  Rows: &mut usize,                       // OUT
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.SetMode()
pub type EFI_TEXT_SET_MODE = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  ModeNumber: usize,                      // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.SetAttribute()
pub type EFI_TEXT_SET_ATTRIBUTE = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  Attribute: usize,                       // IN
) -> EFI_STATUS;

//*******************************************************
// Attributes
//*******************************************************
pub const EFI_BLACK: u8 = 0x00;
pub const EFI_BLUE: u8 = 0x01;
pub const EFI_GREEN: u8 = 0x02;
pub const EFI_CYAN: u8 = 0x03;
pub const EFI_RED: u8 = 0x04;
pub const EFI_MAGENTA: u8 = 0x05;
pub const EFI_BROWN: u8 = 0x06;
pub const EFI_LIGHTGRAY: u8 = 0x07;
pub const EFI_BRIGHT: u8 = 0x08;
pub const EFI_LIGHTBLUE: u8 = 0x09;
pub const EFI_LIGHTGREEN: u8 = 0x0A;
pub const EFI_LIGHTCYAN: u8 = 0x0B;
pub const EFI_LIGHTRED: u8 = 0x0C;
pub const EFI_LIGHTMAGENTA: u8 = 0x0D;
pub const EFI_YELLOW: u8 = 0x0E;
pub const EFI_WHITE: u8 = 0x0F;
pub const EFI_BACKGROUND_BLACK: u8 = 0x00;
pub const EFI_BACKGROUND_BLUE: u8 = 0x10;
pub const EFI_BACKGROUND_GREEN: u8 = 0x20;
pub const EFI_BACKGROUND_CYAN: u8 = 0x30;
pub const EFI_BACKGROUND_RED: u8 = 0x40;
pub const EFI_BACKGROUND_MAGENTA: u8 = 0x50;
pub const EFI_BACKGROUND_BROWN: u8 = 0x60;
pub const EFI_BACKGROUND_LIGHTGRAY: u8 = 0x70;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen()
pub type EFI_TEXT_CLEAR_SCREEN = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
) -> EFI_STATUS;

// FI_SIMPLE_TEXT_OUTPUT_PROTOCOL.SetCursorPosition()
pub type EFI_TEXT_SET_CURSOR_POSITION = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  Column: usize,                          // IN
  Row: usize,                             // IN
) -> EFI_STATUS;

// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.EnableCursor()
pub type EFI_TEXT_ENABLE_CURSOR = extern "C" fn(
  This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, // IN
  Visible: bool,                          // IN
) -> EFI_STATUS;

// EFI_SIMPLE_POINTER_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_POINTER_PROTOCOL {
  pub Reset: EFI_SIMPLE_POINTER_RESET,
  pub GetState: EFI_SIMPLE_POINTER_GET_STATE,
  pub WaitForInput: EFI_EVENT,
  pub Mode: EFI_SIMPLE_POINTER_MODE,
}

#[repr(C)]
pub struct EFI_SIMPLE_POINTER_MODE {
  pub ResolutionX: u64,
  pub ResolutionY: u64,
  pub ResolutionZ: u64,
  pub LeftButton: bool,
  pub RightButton: bool,
}

// EFI_SIMPLE_POINTER_PROTOCOL.Reset()
pub type EFI_SIMPLE_POINTER_RESET = extern "C" fn(
  This: &EFI_SIMPLE_POINTER_PROTOCOL, // IN
  ExtendedVerification: bool,         // IN
) -> EFI_STATUS;

// EFI_SIMPLE_POINTER_PROTOCOL.GetState()
pub type EFI_SIMPLE_POINTER_GET_STATE = extern "C" fn(
  This: &EFI_SIMPLE_POINTER_PROTOCOL,   // IN
  State: &mut EFI_SIMPLE_POINTER_STATE, // IN OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_SIMPLE_POINTER_STATE {
  pub RelativeMovementX: i32,
  pub RelativeMovementY: i32,
  pub RelativeMovementZ: i32,
  pub LeftButton: bool,
  pub RightButton: bool,
}

// EFI_ABSOLUTE_POINTER_PROTOCOL
#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_PROTOCOL {
  pub Reset: EFI_ABSOLUTE_POINTER_RESET,
  pub GetState: EFI_ABSOLUTE_POINTER_GET_STATE,
  pub WaitForInput: EFI_EVENT,
  pub Mode: EFI_ABSOLUTE_POINTER_MODE,
}

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_MODE {
  pub AbsoluteMinX: u64,
  pub AbsoluteMinY: u64,
  pub AbsoluteMinZ: u64,
  pub AbsoluteMaxX: u64,
  pub AbsoluteMaxY: u64,
  pub AbsoluteMaxZ: u64,
  pub Attributes: u32,
}

pub const EFI_ABSP_SupportsAltActive: u32 = 0x00000001;
pub const EFI_ABSP_SupportsPressureAsZ: u32 = 0x00000002;

// EFI_ABSOLUTE_POINTER_PROTOCOL.Reset()
pub type EFI_ABSOLUTE_POINTER_RESET = extern "C" fn(
  This: &EFI_ABSOLUTE_POINTER_PROTOCOL, // IN
  ExtendedVerification: bool,           // IN
) -> EFI_STATUS;

// EFI_ABSOLUTE_POINTER_PROTOCOL.GetState()
pub type EFI_ABSOLUTE_POINTER_GET_STATE = extern "C" fn(
  This: &EFI_ABSOLUTE_POINTER_PROTOCOL,   // IN
  State: &mut EFI_ABSOLUTE_POINTER_STATE, // IN OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_STATE {
  pub CurrentX: u64,
  pub CurrentY: u64,
  pub CurrentZ: u64,
  pub ActiveButtons: u32,
}

pub const EFI_ABSP_TouchActive: u32 = 0x00000001;
pub const EFI_ABS_AltActive: u32 = 0x00000002;

// EFI_SERIAL_IO_PROTOCOL
#[repr(C)]
pub struct EFI_SERIAL_IO_PROTOCOL<'a> {
  pub Revision: u32,
  pub Reset: EFI_SERIAL_RESET,
  pub SetAttributes: EFI_SERIAL_SET_ATTRIBUTES,
  pub SetControl: EFI_SERIAL_SET_CONTROL_BITS,
  pub GetControl: EFI_SERIAL_GET_CONTROL_BITS,
  pub Write: EFI_SERIAL_WRITE,
  pub Read: EFI_SERIAL_READ,
  pub Mode: &'a mut SERIAL_IO_MODE,
  pub DeviceTypeGuid: &'a EFI_GUID, // Revision 1.1
}

#[repr(C)]
pub struct SERIAL_IO_MODE {
  pub ControlMask: u32,
  // current Attributes
  pub Timeout: u32,
  pub BaudRate: u64,
  pub ReceiveFifoDepth: u32,
  pub DataBits: u32,
  pub Parity: u32,
  pub StopBits: u32,
}

#[repr(C)]
pub enum EFI_PARITY_TYPE {
  DefaultParity,
  NoParity,
  EvenParity,
  OddParity,
  MarkParity,
  SpaceParity,
}

#[repr(C)]
pub enum EFI_STOP_BITS_TYPE {
  DefaultStopBits,
  OneStopBit,      // 1 stop bit
  OneFiveStopBits, // 1.5 stop bits
  TwoStopBits,     // 2 stop bits
}

// EFI_SERIAL_IO_PROTOCOL.Reset()
pub type EFI_SERIAL_RESET = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_SERIAL_IO_PROTOCOL.SetAttributes()
pub type EFI_SERIAL_SET_ATTRIBUTES = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
  BaudRate: u64,                 // IN
  ReceiveFifoDepth: u32,         // IN
  Timeout: u32,                  // IN
  Parity: EFI_PARITY_TYPE,       // IN
  DataBits: u8,                  // IN
  StopBits: EFI_STOP_BITS_TYPE,  // IN
) -> EFI_STATUS;

// EFI_SERIAL_IO_PROTOCOL.SetControl()
pub type EFI_SERIAL_SET_CONTROL_BITS = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
  Control: u32,                  // IN
) -> EFI_STATUS;

pub const EFI_SERIAL_CLEAR_TO_SEND: u16 = 0x0010;
pub const EFI_SERIAL_DATA_SET_READY: u16 = 0x0020;
pub const EFI_SERIAL_RING_INDICATE: u16 = 0x0040;
pub const EFI_SERIAL_CARRIER_DETECT: u16 = 0x0080;
pub const EFI_SERIAL_REQUEST_TO_SEND: u16 = 0x0002;
pub const EFI_SERIAL_DATA_TERMINAL_READY: u16 = 0x0001;
pub const EFI_SERIAL_INPUT_BUFFER_EMPTY: u16 = 0x0100;
pub const EFI_SERIAL_OUTPUT_BUFFER_EMPTY: u16 = 0x0200;
pub const EFI_SERIAL_HARDWARE_LOOPBACK_ENABLE: u16 = 0x1000;
pub const EFI_SERIAL_SOFTWARE_LOOPBACK_ENABLE: u16 = 0x2000;
pub const EFI_SERIAL_HARDWARE_FLOW_CONTROL_ENABLE: u16 = 0x4000;

// EFI_SERIAL_IO_PROTOCOL.GetControl()
pub type EFI_SERIAL_GET_CONTROL_BITS = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
  Control: &mut u32,             // OUT
) -> EFI_STATUS;

// EFI_SERIAL_IO_PROTOCOL.Write()
pub type EFI_SERIAL_WRITE = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
  BufferSize: &mut usize,        // IN OUT
  Buffer: VOID_PTR,              // IN
) -> EFI_STATUS;

// EFI_SERIAL_IO_PROTOCOL.Read()
pub type EFI_SERIAL_READ = extern "C" fn(
  This: &EFI_SERIAL_IO_PROTOCOL, // IN
  BufferSize: &mut usize,        // IN OUT
  Buffer: VOID_PTR,              // OUT
) -> EFI_STATUS;

// EFI_GRAPHICS_OUTPUT_PROTOCOL
#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL<'a> {
  pub QueryMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE,
  pub SetMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE,
  pub Blt: EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT,
  pub Mode: &'a EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE<'a>,
}

#[repr(C)]
pub struct EFI_PIXEL_BITMASK {
  pub RedMask: u32,
  pub GreenMask: u32,
  pub BlueMask: u32,
  pub ReservedMask: u32,
}

use core::fmt::Debug;
#[repr(C)]
#[derive(Debug)]
pub enum EFI_GRAPHICS_PIXEL_FORMAT {
  PixelRedGreenBlueReserved8BitPerColor,
  PixelBlueGreenRedReserved8BitPerColor,
  PixelBitMask,
  PixelBltOnly,
  PixelFormatMax,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
  pub Version: u32,
  pub HorizontalResolution: u32,
  pub VerticalResolution: u32,
  pub PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
  pub PixelInformation: EFI_PIXEL_BITMASK,
  pub PixelsPerScanLine: u32,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE<'a> {
  pub MaxMode: u32,
  pub Mode: u32,
  pub Info: &'a EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
  pub SizeOfInfo: usize,
  pub FrameBufferBase: EFI_PHYSICAL_ADDRESS,
  pub FrameBufferSize: usize,
}

// EFI_GRAPHICS_OUTPUT_PROTOCOL.QueryMode()
pub type EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE = extern "C" fn(
  This: &EFI_GRAPHICS_OUTPUT_PROTOCOL,              // IN
  ModeNumber: u32,                                  // IN
  SizeOfInfo: &mut usize,                           // OUT
  Info: &mut &EFI_GRAPHICS_OUTPUT_MODE_INFORMATION, // OUT
) -> EFI_STATUS;

// EFI_GRAPHICS_OUTPUT_PROTOCOL.SetMode()
pub type EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE = extern "C" fn(
  This: &EFI_GRAPHICS_OUTPUT_PROTOCOL, // IN
  ModeNumber: u32,                     // IN
) -> EFI_STATUS;

// EFI_GRAPHICS_OUTPUT_PROTOCOL.Blt()
#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_BLT_PIXEL {
  pub Blue: u8,
  pub Green: u8,
  pub Red: u8,
  pub Reserved: u8,
}

#[repr(C)]
pub enum EFI_GRAPHICS_OUTPUT_BLT_OPERATION {
  EfiBltVideoFill,
  EfiBltVideoToBltBuffer,
  EfiBltBufferToVideo,
  EfiBltVideoToVideo,
  EfiGraphicsOutputBltOperationMax,
}

pub type EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT = extern "C" fn(
  This: &EFI_GRAPHICS_OUTPUT_PROTOCOL,                   // IN
  BltBuffer: Option<&mut EFI_GRAPHICS_OUTPUT_BLT_PIXEL>, // IN OUT
  BltOperation: EFI_GRAPHICS_OUTPUT_BLT_OPERATION,       // IN
  SourceX: usize,                                        // IN
  SourceY: usize,                                        // IN
  DestinationX: usize,                                   // IN
  DestinationY: usize,                                   // IN
  Width: usize,                                          // IN
  Height: usize,                                         // IN
  Delta: Option<usize>,                                  // IN
) -> EFI_STATUS;

// EFI_EDID_DISCOVERED_PROTOCOL
#[repr(C)]
pub struct EFI_EDID_DISCOVERED_PROTOCOL<'a> {
  pub SizeOfEdid: u32,
  pub Edid: &'a mut u8,
}

// EFI_EDID_ACTIVE_PROTOCOL
#[repr(C)]
pub struct EFI_EDID_ACTIVE_PROTOCOL<'a> {
  pub SizeOfEdid: u32,
  pub Edid: &'a mut u8,
}

// EFI_EDID_OVERRIDE_PROTOCOL
#[repr(C)]
pub struct EFI_EDID_OVERRIDE_PROTOCOL {
  pub GetEdid: EFI_EDID_OVERRIDE_PROTOCOL_GET_EDID,
}

// EFI_EDID_OVERRIDE_PROTOCOL.GetEdid()
pub type EFI_EDID_OVERRIDE_PROTOCOL_GET_EDID = extern "C" fn(
  This: &EFI_EDID_OVERRIDE_PROTOCOL, // IN
  ChildHandle: &EFI_HANDLE,          // IN
  Attributes: &mut u32,              // OUT
  EdidSize: &mut usize,              // IN OUT
  Edid: &mut &mut u8,                // IN OUT
) -> EFI_STATUS;

pub const EFI_EDID_OVERRIDE_DONT_OVERRIDE: u8 = 0x01;
pub const EFI_EDID_OVERRIDE_ENABLE_HOT_PLUG: u8 = 0x02;
