use elf::*;

#[test]
fn header_size() {
  use std::mem::size_of;
  assert_eq!(size_of::<FileHeader>(), 64);
  assert_eq!(size_of::<ProgramHeader>(), 56);
  assert_eq!(size_of::<SectionHeader>(), 64);
}
