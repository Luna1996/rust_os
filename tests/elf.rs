#![allow(safe_packed_borrows)]

use elf::*;
use std::mem::size_of;

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
fn header_size() {
  assert_eq!(size_of::<FileHeader::<u32>>(), 52);
  assert_eq!(size_of::<FileHeader::<u64>>(), 64);
  assert_eq!(size_of::<ProgramHeader::<u32>>(),32);
  assert_eq!(size_of::<ProgramHeader::<u64>>(),56);
  assert_eq!(size_of::<SectionHeader::<u32>>(),40);
  assert_eq!(size_of::<SectionHeader::<u64>>(),64);
}
