use crate::protocols::device_path_protocols::EFI_DEVICE_PATH_PROTOCOL;
use crate::services::runtime::EFI_TIME;
use crate::types::*;

// EFI_LOAD_FILE_PROTOCOL
#[repr(C)]
pub struct EFI_LOAD_FILE_PROTOCOL {
  pub LoadFile: EFI_LOAD_FILE,
}

// EFI_LOAD_FILE_PROTOCOL.LoadFile()
pub type EFI_LOAD_FILE = extern "C" fn(
  This: &EFI_LOAD_FILE_PROTOCOL,       // IN
  FilePath: &EFI_DEVICE_PATH_PROTOCOL, // IN
  BootPolicy: bool,                    // IN
  BufferSize: &mut usize,              // IN OUT
  Buffer: Option<*const u8>,           // IN
) -> EFI_STATUS;

// EFI_LOAD_FILE2_PROTOCOL
pub type EFI_LOAD_FILE2_PROTOCOL = EFI_LOAD_FILE_PROTOCOL;

// EFI_SIMPLE_FILE_SYSTEM_PROTOCOL
pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_REVISION: u32 = 0x00010000;

#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
  pub Revision: u64,
  pub OpenVolume: EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME,
}

// EFI_SIMPLE_FILE SYSTEM_PROTOCOL.OpenVolume()
pub type EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME = extern "C" fn(
  This: &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, // IN
  Root: &mut &EFI_FILE_PROTOCOL,          // OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL
pub const EFI_FILE_PROTOCOL_REVISION: u32 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: u32 = 0x00020000;
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: u32 = EFI_FILE_PROTOCOL_REVISION2;

#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
  pub Revision: u64,
  pub Open: EFI_FILE_OPEN,
  pub Close: EFI_FILE_CLOSE,
  pub Delete: EFI_FILE_DELETE,
  pub Read: EFI_FILE_READ,
  pub Write: EFI_FILE_WRITE,
  pub GetPosition: EFI_FILE_GET_POSITION,
  pub SetPosition: EFI_FILE_SET_POSITION,
  pub GetInfo: EFI_FILE_GET_INFO,
  pub SetInfo: EFI_FILE_SET_INFO,
  pub Flush: EFI_FILE_FLUSH,
  pub OpenEx: EFI_FILE_OPEN_EX,   // Added for revision 2
  pub ReadEx: EFI_FILE_READ_EX,   // Added for revision 2
  pub WriteE: EFI_FILE_WRITE_EX,  // Added for revision 2
  pub FlushEx: EFI_FILE_FLUSH_EX, // Added for revision 2
}

// EFI_FILE_PROTOCOL.Open()
pub type EFI_FILE_OPEN = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,           // IN
  NewHandle: &mut &EFI_FILE_PROTOCOL, // OUT
  FileName: &[u16],                   // IN
  OpenMode: u64,                      // IN
  Attributes: u64,                    // IN
) -> EFI_STATUS;

// Open Modes
pub const EFI_FILE_MODE_READ: u64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: u64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: u64 = 0x8000000000000000;
// File Attributes
pub const EFI_FILE_READ_ONLY: u64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: u64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: u64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: u64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: u64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: u64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: u64 = 0x0000000000000037;

// EFI_FILE_PROTOCOL.Close()
pub type EFI_FILE_CLOSE = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.Delete()
pub type EFI_FILE_DELETE = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.Read()
pub type EFI_FILE_READ = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
  BufferSize: &mut usize,   // IN OUT
  Buffer: *mut u8,          // OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.Write()
pub type EFI_FILE_WRITE = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
  BufferSize: &mut usize,   // IN OUT
  Buffer: *const u8,        // IN
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.OpenEx()
pub type EFI_FILE_OPEN_EX = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,           // IN
  NewHandle: &mut &EFI_FILE_PROTOCOL, // OUT
  FileName: &[u16],                   // IN
  OpenMode: u64,                      // IN
  Attributes: u64,                    // IN
  Token: &mut EFI_FILE_IO_TOKEN,      // IN OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
  pub Event: EFI_EVENT,
  pub Status: EFI_STATUS,
  pub BufferSize: usize,
  pub Buffer: *const u8,
}

// EFI_FILE_PROTOCOL.ReadEx()
pub type EFI_FILE_READ_EX = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,      // IN
  Token: &mut EFI_FILE_IO_TOKEN, // IN OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.WriteEx()
pub type EFI_FILE_WRITE_EX = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,      // IN
  Token: &mut EFI_FILE_IO_TOKEN, // IN OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.FlushEx()
pub type EFI_FILE_FLUSH_EX = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,      // IN
  Token: &mut EFI_FILE_IO_TOKEN, // IN OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.SetPosition()
pub type EFI_FILE_SET_POSITION = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
  Position: u64,            // IN
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.GetPosition()
pub type EFI_FILE_GET_POSITION = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
  Position: &mut u64,       // OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.GetInfo()
pub type EFI_FILE_GET_INFO = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,   // IN
  InformationType: &EFI_GUID, // IN
  BufferSize: &mut usize,     // IN OUT
  Buffer: *const u8,          // OUT
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.SetInfo()
pub type EFI_FILE_SET_INFO = extern "C" fn(
  This: &EFI_FILE_PROTOCOL,   // IN
  InformationType: &EFI_GUID, // IN
  BufferSize: usize,          // IN
  Buffer: *const u8,          // IN
) -> EFI_STATUS;

// EFI_FILE_PROTOCOL.Flush()
pub type EFI_FILE_FLUSH = extern "C" fn(
  This: &EFI_FILE_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_FILE_INFO
#[repr(C)]
pub struct EFI_FILE_INFO<'a> {
  pub Size: u64,
  pub FileSize: u64,
  pub PhysicalSize: u64,
  pub CreateTime: EFI_TIME,
  pub LastAccessTime: EFI_TIME,
  pub ModificationTime: EFI_TIME,
  pub Attribute: u64,
  pub FileName: &'a [u16],
}

// EFI_FILE_SYSTEM_INFO
#[repr(C)]
pub struct EFI_FILE_SYSTEM_INFO<'a> {
  pub Size: u64,
  pub ReadOnly: bool,
  pub VolumeSize: u64,
  pub FreeSpace: u64,
  pub BlockSize: u32,
  pub VolumeLabel: &'a [u16],
}

// EFI_FILE_SYSTEM_VOLUME_LABEL
#[repr(C)]
pub struct EFI_FILE_SYSTEM_VOLUME_LABEL<'a> {
  pub VolumeLabel: &'a [u16],
}

// EFI_TAPE_IO_PROTOCOL
#[repr(C)]
pub struct EFI_TAPE_IO_PROTOCOL {
  pub TapeRead: EFI_TAPE_READ,
  pub TapeWrite: EFI_TAPE_WRITE,
  pub TapeRewind: EFI_TAPE_REWIND,
  pub TapeSpace: EFI_TAPE_SPACE,
  pub TapeWriteFM: EFI_TAPE_WRITEFM,
  pub TapeReset: EFI_TAPE_RESET,
}

// EFI_TAPE_IO_PROTOCOL.TapeRead()
pub type EFI_TAPE_READ = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
  BufferSize: &mut usize,      // IN OUT
  Buffer: *const u8,           // OUT
) -> EFI_STATUS;

// EFI_TAPE_IO_PROTOCOL.TapeWrite()
pub type EFI_TAPE_WRITE = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
  BufferSize: &mut usize,      // IN
  Buffer: *const u8,           // IN
) -> EFI_STATUS;

// EFI_TAPE_IO_PROTOCOL.TapeRewind()
pub type EFI_TAPE_REWIND = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_TAPE_IO_PROTOCOL.TapeSpace()
pub type EFI_TAPE_SPACE = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
  Direction: isize,            // IN
  Type: usize,                 // IN
) -> EFI_STATUS;

// EFI_TAPE_IO_PROTOCOL.TapeWriteFM()
pub type EFI_TAPE_WRITEFM = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
  Count: usize,                // IN
) -> EFI_STATUS;

// EFI_TAPE_IO_PROTOCOL.TapeReset()
pub type EFI_TAPE_RESET = extern "C" fn(
  This: &EFI_TAPE_IO_PROTOCOL, // IN
  ExtendedVerification: bool,  // IN
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_TAPE_HEADER<'a> {
  Signature: u64,
  Revision: u32,
  BootDescSize: u32,
  BootDescCRC: u32,
  TapeGUID: EFI_GUID,
  TapeType: EFI_GUID,
  TapeUnique: EFI_GUID,
  BLLocation: u32,
  BLBlocksize: u32,
  BLFilesize: u32,
  OSVersion: &'a [u8; 40],
  AppVersion: &'a [u8; 40],
  CreationDate: &'a [u8; 10],
  CreationTime: &'a [u8; 10],
  SystemName: &'a [u8; 256], // UTF-8
  TapeTitle: &'a [u8; 120],  // UTF-8
  pad: &'a [u8; 468],        // pad to 1024
}

// EFI_DISK_IO_PROTOCOL
pub const EFI_DISK_IO_PROTOCOL_REVISION: u32 = 0x00010000;

#[repr(C)]
pub struct EFI_DISK_IO_PROTOCOL {
  pub Revision: u64,
  pub ReadDisk: EFI_DISK_READ,
  pub WriteDisk: EFI_DISK_WRITE,
}

// EFI_DISK_IO_PROTOCOL.ReadDisk()
pub type EFI_DISK_READ = extern "C" fn(
  This: &EFI_DISK_IO_PROTOCOL, // IN
  MediaId: u32,                // IN
  Offset: u64,                 // IN
  BufferSize: usize,           // IN
  Buffer: *const u8,           // OUT
) -> EFI_STATUS;

// EFI_DISK_IO_PROTOCOL.WriteDisk()
pub type EFI_DISK_WRITE = extern "C" fn(
  This: &EFI_DISK_IO_PROTOCOL, // IN
  MediaId: u32,                // IN
  Offset: u64,                 // IN
  BufferSize: usize,           // IN
  Buffer: *const u8,           // IN
) -> EFI_STATUS;

// EFI_DISK_IO2_PROTOCOL
pub const EFI_DISK_IO2_PROTOCOL_REVISION: u32 = 0x00020000;

#[repr(C)]
pub struct EFI_DISK_IO2_PROTOCOL {
  pub Revision: u64,
  pub Cancel: EFI_DISK_CANCEL_EX,
  pub ReadDiskEx: EFI_DISK_READ_EX,
  pub WriteDiskEx: EFI_DISK_WRITE_EX,
  pub FlushDiskEx: EFI_DISK_FLUSH_EX,
}

// EFI_DISK_IO2_PROTOCOL.Cancel()
pub type EFI_DISK_CANCEL_EX = extern "C" fn(
  This: &EFI_DISK_IO2_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_DISK_IO2_PROTOCOL.ReadDiskEx()
pub type EFI_DISK_READ_EX = extern "C" fn(
  This: &EFI_DISK_IO2_PROTOCOL,   // IN
  MediaId: u32,                   // IN
  Offset: u64,                    // IN
  Token: &mut EFI_DISK_IO2_TOKEN, // IN OUT
  BufferSize: usize,              // IN
  Buffer: *const u8,              // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_DISK_IO2_TOKEN {
  pub Event: EFI_EVENT,
  pub TransactionStatus: EFI_STATUS,
}

// EFI_DISK_IO2_PROTOCOL.WriteDiskEx()
pub type EFI_DISK_WRITE_EX = extern "C" fn(
  This: &EFI_DISK_IO2_PROTOCOL,   // IN
  MediaId: u32,                   // IN
  Offset: u64,                    // IN
  Token: &mut EFI_DISK_IO2_TOKEN, // IN OUT
  BufferSize: usize,              // IN
  Buffer: *const u8,              // IN
) -> EFI_STATUS;

// EFI_DISK_IO2_PROTOCOL.FlushDiskEx()
pub type EFI_DISK_FLUSH_EX = extern "C" fn(
  This: &EFI_DISK_IO2_PROTOCOL,   // IN
  Token: &mut EFI_DISK_IO2_TOKEN, // IN OUT
) -> EFI_STATUS;

// EFI_BLOCK_IO_PROTOCOL
pub const EFI_BLOCK_IO_PROTOCOL_REVISION2: u32 = 0x00020001;
pub const EFI_BLOCK_IO_PROTOCOL_REVISION3: u32 = ((2 << 16) | (31));

#[repr(C)]
pub struct EFI_BLOCK_IO_PROTOCOL<'a> {
  pub Revision: u64,
  pub Media: &'a EFI_BLOCK_IO_MEDIA,
  pub Reset: EFI_BLOCK_RESET,
  pub ReadBlocks: EFI_BLOCK_READ,
  pub WriteBlocks: EFI_BLOCK_WRITE,
  pub FlushBlocks: EFI_BLOCK_FLUSH,
}

// EFI_BLOCK_IO_MEDIA
#[repr(C)]
pub struct EFI_BLOCK_IO_MEDIA {
  pub MediaId: u32,
  pub RemovableMedia: bool,
  pub MediaPresent: bool,
  pub LogicalPartition: bool,
  pub ReadOnly: bool,
  pub WriteCaching: bool,
  pub BlockSize: u32,
  pub IoAlign: u32,
  pub LastBlock: EFI_LBA,
  pub LowestAlignedLba: EFI_LBA,             //added in Revision 2
  pub LogicalBlocksPerPhysicalBlock: u32,    //added in Revision 2
  pub OptimalTransferLengthGranularity: u32, // added in Revision 3
}
// EFI_LBA
pub type EFI_LBA = u64;

// EFI_BLOCK_IO_PROTOCOL.Reset()
pub type EFI_BLOCK_RESET = extern "C" fn(
  This: &EFI_BLOCK_IO_PROTOCOL, // IN
  ExtendedVerification: bool,   // IN
) -> EFI_STATUS;

// EFI_BLOCK_IO_PROTOCOL.ReadBlocks()
pub type EFI_BLOCK_READ = extern "C" fn(
  This: &EFI_BLOCK_IO_PROTOCOL, // IN
  MediaId: u32,                 // IN
  LBA: EFI_LBA,                 // IN
  BufferSize: usize,            // IN
  Buffer: *const u8,            // OUT
) -> EFI_STATUS;

// EFI_BLOCK_IO_PROTOCOL.WriteBlocks()
pub type EFI_BLOCK_WRITE = extern "C" fn(
  This: &EFI_BLOCK_IO_PROTOCOL, // IN
  MediaId: u32,                 // IN
  LBA: EFI_LBA,                 // IN
  BufferSize: usize,            // IN
  Buffer: *const u8,            // IN
) -> EFI_STATUS;

// EFI_BLOCK_IO_PROTOCOL.FlushBlocks()
pub type EFI_BLOCK_FLUSH = extern "C" fn(
  This: &EFI_BLOCK_IO_PROTOCOL, // IN
) -> EFI_STATUS;

// EFI_BLOCK_IO2_PROTOCOL

#[repr(C)]
pub struct EFI_BLOCK_IO2_PROTOCOL<'a> {
  pub Media: &'a EFI_BLOCK_IO_MEDIA,
  pub Reset: EFI_BLOCK_RESET_EX,
  pub ReadBlocksEx: EFI_BLOCK_READ_EX,
  pub WriteBlocksEx: EFI_BLOCK_WRITE_EX,
  pub FlushBlocksEx: EFI_BLOCK_FLUSH_EX,
}

// EFI_BLOCK_IO2_PROTOCOL.Reset()
pub type EFI_BLOCK_RESET_EX = extern "C" fn(
  This: &EFI_BLOCK_IO2_PROTOCOL, // IN
  ExtendedVerification: bool,    // IN
) -> EFI_STATUS;

// EFI_BLOCK_IO2_PROTOCOL.ReadBlocksEx()
pub type EFI_BLOCK_READ_EX = extern "C" fn(
  This: &EFI_BLOCK_IO2_PROTOCOL,   // IN
  MediaId: u32,                    // IN
  LBA: EFI_LBA,                    // IN
  Token: &mut EFI_BLOCK_IO2_TOKEN, // IN OUT
  BufferSize: usize,               // IN
  Buffer: *const u8,               // OUT
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_BLOCK_IO2_TOKEN {
  pub Event: EFI_EVENT,
  pub TransactionStatus: EFI_STATUS,
}

// EFI_BLOCK_IO2_PROTOCOL.WriteBlocksEx()
pub type EFI_BLOCK_WRITE_EX = extern "C" fn(
  This: &EFI_BLOCK_IO2_PROTOCOL,   // IN
  MediaId: u32,                    // IN
  LBA: EFI_LBA,                    // IN
  Token: &mut EFI_BLOCK_IO2_TOKEN, // IN OUT
  BufferSize: usize,               // IN
  Buffer: *const u8,               // IN
) -> EFI_STATUS;

// EFI_BLOCK_IO2_PROTOCOL.FlushBlocksEx()
pub type EFI_BLOCK_FLUSH_EX = extern "C" fn(
  This: &EFI_BLOCK_IO2_PROTOCOL,   // IN
  Token: &mut EFI_BLOCK_IO2_TOKEN, // IN OUT
) -> EFI_STATUS;

// EFI_BLOCK_IO_CRYPTO_PROTOCOL
#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_PROTOCOL<'a> {
  pub Media: &'a EFI_BLOCK_IO_MEDIA,
  // pub Reset: EFI_BLOCK_IO_CRYPTO_RESET,
  // pub GetCapabilities: EFI_BLOCK_IO_CRYPTO_GET_CAPABILITIES,
  // pub SetConfiguration: EFI_BLOCK_IO_CRYPTO_SET_CONFIGURATION,
  // pub GetConfiguration: EFI_BLOCK_IO_CRYPTO_GET_CONFIGURATION,
  // pub ReadExtended: EFI_BLOCK_IO_CRYPTO_READ_DEVICE_EXTENDED,
  // pub WriteExtended: EFI_BLOCK_IO_CRYPTO_WRITE_DEVICE_EXTENDED,
  // pub FlushBlocks: EFI_BLOCK_IO_CRYPTO_FLUSH,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_CAPABILITY {
  pub Algorithm: EFI_GUID,
  pub KeySize: u64,
  pub CryptoBlockSizeBitMask: u64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT_AES_XTS {
  pub Header: EFI_BLOCK_IO_CRYPTO_IV_INPUT,
  pub CryptoBlockNumber: u64,
  pub CryptoBlockByteSize: u64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT_AES_CBC_MICROSOFT_BITLOCKER {
  pub Header: EFI_BLOCK_IO_CRYPTO_IV_INPUT,
  pub CryptoBlockByteOffset: u64,
  pub CryptoBlockByteSize: u64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT {
  pub InputSize: u64,
}
