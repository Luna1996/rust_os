use elf::*;

#[test]
fn header_size() {
  use std::mem::size_of;
  assert_eq!(size_of::<FileHeader::<u64>>(), 64);
  assert_eq!(size_of::<ProgramHeader::<u64>>(), 56);
  assert_eq!(size_of::<SectionHeader::<u64>>(), 64);
}

#[test]
fn parser_test() {
  use std::{
    fs::{read, write},
    slice::from_raw_parts,
  };
  let raw = read("../build/EFI/kernel").unwrap();
  let elf_file: ELFFile<u64> = ELFFile::parse(raw.as_slice()).unwrap();
  println!("entry(0x{:016X})", elf_file.fh.e_entry);
  for ph in elf_file.phs {
    println!(
      "type(0x{:08X}), vaddr(0x{:016X})",
      ph.p_type as u32, ph.p_vaddr
    );
  }
  // for sh in elf_file.shs {
  //   println!(
  //     "section: {} 0x{:0>16X}",
  //     elf_file.getSectionName(sh),
  //     sh.sh_addr
  //   );
  // }
}
