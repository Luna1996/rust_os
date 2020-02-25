#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::fmt::Debug;
use core::mem::transmute;

/// doc: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub const MAGIC: &[u8] = "\x7fELF".as_bytes();

#[derive(Debug)]
#[repr(u8)]
pub enum MODE {
  BIT32 = 1,
  BIT64 = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum ENDI {
  LITTLE = 1,
  BIG = 2,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(packed(2))]
pub struct FileHeader {
  pub e_ident: E_IDENT,
  pub e_type: FTYPE,
  pub e_machine: MACHINE,
  pub e_version: u32,
  pub e_entry: usize,
  pub e_phoff: usize,
  pub e_shoff: usize,
  pub e_flags: u32,
  pub e_ehsize: u16,
  pub e_phentsize: u16,
  pub e_phnum: u16,
  pub e_shentsize: u16,
  pub e_shnum: u16,
  pub e_shstrndx: u16,
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct ProgramHeader {
  pub p_type: usize,
  pub p_offset: usize,
  pub p_vaddr: usize,
  pub p_paddr: usize,
  pub p_filesz: usize,
  pub p_memsz: usize,
  _pad: u32,
  pub p_align: u32,
}

#[derive(Debug)]
#[repr(packed(4))]
pub struct SectionHeader {
  pub sh_name: u32,
  pub sh_type: u32,
  pub sh_flags: usize,
  pub sh_addr: usize,
  pub sh_offset: usize,
  pub sh_size: usize,
  pub sh_link: u32,
  pub sh_info: u32,
  pub sh_addralign: usize,
  pub sh_entsize: usize,
}

#[derive(Debug)]
pub struct ELFFile<'a> {
  pub raw: *const u8,
  pub fh: &'a FileHeader,
  // phs: *const ProgramHeader<T>,
  // shs: *const SectionHeader<T>,
}

pub fn check(raw: &[u8]) -> Result<MODE, &str> {
  if raw[0] != MAGIC[0] || raw[1] != MAGIC[1] || raw[2] != MAGIC[2] || raw[3] != MAGIC[3] {
    Ok(unsafe { transmute(raw[5]) })
  } else {
    Err("ELF magic number check fail!")
  }
}

impl<'a> ELFFile<'a> {
  pub fn new(raw: *const u8) -> ELFFile<'a> {
    ELFFile::<'a> {
      raw,
      fh: unsafe { &*(raw as *const FileHeader) },
    }
  }

  pub fn load(&mut self, addr: *mut u8, size: usize) {
    unsafe { addr.copy_from(self.raw, size) };
    self.raw = addr as *const u8;
  }

  pub fn getEntry(&self) -> Result<*const u8, &str> {
    Ok(unsafe { self.raw.offset(self.fh.e_entry as isize) })
  }
}
