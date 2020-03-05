//╔═══════════╗
//║  Helpers  ║
//╚═══════════╝
pub unsafe fn s16(s8: &str) -> *const u16 {
  let n = s8.len() + 1;
  let buf = efi_alloc!(u16, n);
  buf[n - 1] = 0;
  for (i, c) in s8.encode_utf16().enumerate() {
    buf[i] = c;
  }
  &buf[0]
}

use core::str::from_utf8_unchecked;
pub unsafe fn s8(s16: &*const u16) -> &str {
  let n = {
    let mut n = 0;
    while *(s16.add(n)) != 0 {
      n = n + 1;
    }
    n
  };
  let buf = efi_alloc![u8, n];
  // BUFFERS.push(buf8);
  for i in 0..n {
    buf[i] = *(s16.add(i)) as u8;
  }
  from_utf8_unchecked(buf)
}

//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
use uefi::efi_system_table::EFI_SYSTEM_TABLE;
pub unsafe fn init(st: &EFI_SYSTEM_TABLE) {
  let cout = st.ConOut;
  let max_mode = (cout.Mode.MaxMode - 1) as usize;
  (cout.ClearScreen)(cout);
  (cout.SetMode)(cout, max_mode);
}
