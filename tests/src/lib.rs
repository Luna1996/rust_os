#![cfg(test)]
#![allow(safe_packed_borrows, unused_imports)]
#![feature(type_name_of_val)]
mod test_elf;
mod test_uefi;

#[test]
fn raw_to_struct() {
  #[repr(packed(1))]
  struct A {
    a1: u8,
    a2: u16,
    a3: u8,
    a4: u8,
  };
  let data: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let s: &A = unsafe { &(*((&data as *const u8) as *const A)) };
  println!("{},{},{},{}", s.a1, s.a2, s.a3, s.a4);
}

#[test]
fn slice_size() {
  use std::mem::size_of;
  println!("&[u8] : {}", size_of::<&[u8]>());
  println!("*[u8] : {}", size_of::<*const [u8]>());
  println!("&u8 : {}", size_of::<&u8>());
  println!("*u8 : {}", size_of::<*const u8>());
  println!("usize : {}", size_of::<usize>());
  println!("*usize : {}", size_of::<*const usize>());
}

#[test]
fn ref_vs_pointer() {
  let a: [u8; 4] = [1, 2, 3, 5];
  println!("0x{:0>16X}", (&a as *const u8) as usize);
  println!("0x{:0>16X}", (&a[0] as *const u8) as usize);
}

#[test]
fn slice_ref() {
  use std::any::type_name_of_val;
  let a: [u8; 4] = [1, 2, 3, 4];
  let b = &a[..];
  println!("{}", type_name_of_val(&a));
  println!("{}", type_name_of_val(b));
  assert_eq!(&a as *const u8, &b[0] as *const u8);
}

#[test]
fn pointer_offset() {
  unsafe {
    let a = 0;
    let b = &a as *const i32;
    let c = b.offset(1);
    assert_ne!(b, c);
  }
}

#[test]
fn tuple_test() {
  let mut a: (i32, i32) = (0, 0);
  fn f(n: &mut i32) {
    *n = 1
  };
  f(&mut a.0);
  println!("{}", a.0);
}
