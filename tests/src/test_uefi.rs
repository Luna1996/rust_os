#[test]
fn guid_data_layout() {
  use std::mem::{size_of, transmute};
  #[repr(C)]
  struct GUID(u32, [u16; 2], [u8; 8]);
  let guid: u128 = unsafe {
    transmute(GUID(
      0x9042a9de,
      [0x23dc, 0x4a38],
      [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
    ))
  };
  println!("size of GUID: {}", size_of::<GUID>());
  println!("0x{:0>32X}", guid);
}