#![allow(safe_packed_borrows)]

use elf::*;

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
  use std::mem::size_of;
  assert_eq!(size_of::<FileHeader>(), 64);
  assert_eq!(size_of::<ProgramHeader>(), 56);
  assert_eq!(size_of::<SectionHeader>(), 64);
}

#[test]
fn elf_file_test() {
  use core::mem::transmute;
  use std::fs;
  let buf = fs::read("./build/kernel").unwrap();
  let bytes = buf.as_slice();
  println!("{}", bytes);
  check(bytes).unwrap();
  let elf_file = ELFFile::new(unsafe { transmute(&bytes[0]) });
  println!("{:#?}", elf_file.fh);
}
