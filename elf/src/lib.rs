#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// doc: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub const MAGIC: &str = "0x7FELF";

#[repr(u8)]
pub enum MODE {
  BIT32 = 1,
  BIT64 = 2,
}

#[repr(u8)]
pub enum ENDI {
  LITTLE = 1,
  BIG = 2,
}

#[repr(u8)]
pub enum ABI {
  System_V = 0x00,
  HP_UX = 0x01,
  NetBSD = 0x02,
  Linux = 0x03,
  GNU_Hurd = 0x04,
  Solaris = 0x06,
  AIX = 0x07,
  IRIX = 0x08,
  FreeBSD = 0x09,
  Tru64 = 0x0A,
  Novell_Modesto = 0x0B,
  OpenBSD = 0x0C,
  OpenVMS = 0x0D,
  NonStop_Kernel = 0x0E,
  AROS = 0x0F,
  Fenix_OS = 0x10,
  CloudABI = 0x11,
}

#[repr(packed(1))]
pub struct E_IDENT {
  pub EI_MAG: [u8; 4],
  pub EI_CLASS: MODE,
  pub EI_DATA: ENDI,
  pub EI_VERSION: u8,
  pub EI_OSABI: u8,
  pub EI_ABIVERSION: u8,
  _EI_PAD: [u8; 7],
}

#[repr(u16)]
pub enum FTYPE {
  ET_NONE = 0x0000,
  ET_REL = 0x0001,
  ET_EXEC = 0x0002,
  ET_DYN = 0x0003,
  ET_CORE = 0x0004,
  ET_LOOS = 0xfe00,
  ET_HIOS = 0xfeff,
  ET_LOPROC = 0xff00,
  ET_HIPROC = 0xffff,
}

#[repr(u16)]
pub enum MACHINE {
  UNKNOWN = 0x00,
  SPARC = 0x02,
  x86 = 0x03,
  MIPS = 0x08,
  PowerPC = 0x14,
  S390 = 0x16,
  ARM = 0x28,
  SuperH = 0x2A,
  IA_64 = 0x32,
  amd64 = 0x3E,
  AArch64 = 0xB7,
  RISC_V = 0xF3,
}

#[repr(packed(2))]
pub struct FileHeader<T> {
  pub e_ident: E_IDENT,
  pub e_type: FTYPE,
  pub e_machine: MACHINE,
  pub e_version: u32,
  pub e_entry: T,
  pub e_phoff: T,
  pub e_shoff: T,
  pub e_flags: u32,
  pub e_ehsize: u16,
  pub e_phentsize: u16,
  pub e_phnum: u16,
  pub e_shentsize: u16,
  pub e_shnum: u16,
  pub e_shstrndx: u16,
}

#[repr(u32)]
pub enum PTYPE {
  PT_NULL = 0x00000000,
  PT_LOAD = 0x00000001,
  PT_DYNAMIC = 0x00000002,
  PT_INTERP = 0x00000003,
  PT_NOTE = 0x00000004,
  PT_SHLIB = 0x00000005,
  PT_PHDR = 0x00000006,
  PT_TLS = 0x00000007,
  PT_LOOS = 0x60000000,
  PT_HIO = 0x6FFFFFFF,
  PT_LOPRO = 0x70000000,
  PT_HIPRO = 0x7FFFFFFF,
}

#[repr(C)]
pub struct ProgramHeader<T> {
  pub p_type: T,
  pub p_offset: T,
  pub p_vaddr: T,
  pub p_paddr: T,
  pub p_filesz: T,
  pub p_memsz: T,
  _pad: u32,
  pub p_align: u32,
}

#[repr(packed(4))]
pub struct SectionHeader<T> {
  pub sh_name: u32,
  pub sh_type: u32,
  pub sh_flags: T,
  pub sh_addr: T,
  pub sh_offset: T,
  pub sh_size: T,
  pub sh_link: u32,
  pub sh_info: u32,
  pub sh_addralign: T,
  pub sh_entsize: T,
}
