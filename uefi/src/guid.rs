use crate::types::EFI_GUID;
use core::mem::transmute;

impl EFI_GUID {
  pub fn flat(&self) -> &u128 {
    unsafe { transmute(self) }
  }
}

pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(
  0x9042a9de,
  0x23dc,
  0x4a38,
  [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);

pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(
  0x387477c2,
  0x69c7,
  0x11d2,
  [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);