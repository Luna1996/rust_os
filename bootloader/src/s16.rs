use alloc::{string::String, vec::Vec};

pub struct S16(pub Vec<u16>, pub String);

impl From<String> for S16 {
  fn from(s8: String) -> S16 {
    let mut s16 = Vec::with_capacity(s8.len() + 1);
    for c16 in s8.encode_utf16() {
      s16.push(c16);
    }
    s16.push(0);
    S16(s16, s8)
  }
}

impl From<*const u16> for S16 {
  fn from(ptr: *const u16) -> S16 {
    unsafe {
      let mut n = 0;
      while *(ptr.add(n)) != 0 {
        n += 1;
      }
      print!("{}", n);
      let s16 = Vec::from_raw_parts(ptr as *mut u16, n, n);
      let s8 = String::from_utf16(s16.as_slice()).unwrap();
      S16(s16, s8)
    }
  }
}
