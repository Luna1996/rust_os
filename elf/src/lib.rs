#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::{
  mem::{size_of, transmute},
  slice::from_raw_parts,
  str::from_utf8,
};

/// doc: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub struct ELFFile<'a, T> {
  pub raw: &'a [u8],
  shstr: &'a [u8],
  pub fh: &'a FileHeader<T>,
  pub phs: &'a [ProgramHeader<T>],
  pub shs: &'a [SectionHeader<T>],
}

impl<'a, T> ELFFile<'a, T>
where
  T: 'static,
{
  pub fn parse(raw: &'a [u8]) -> Result<ELFFile<'a, T>, &'a str> {
    unsafe {
      if &raw[0..4] == MAGIC {
        if raw[4] << 2 != size_of::<T>() as u8 {
          Err("type missmatch")
        } else {
          let fh: &FileHeader<T> = transmute(&raw[0]);
          let e_phoff: &usize = transmute(&fh.e_phoff);
          let e_shoff: &usize = transmute(&fh.e_shoff);
          let phs = from_raw_parts(
            &raw[*e_phoff] as *const u8 as *const ProgramHeader<T>,
            fh.e_phnum as usize,
          );
          let shs = from_raw_parts(
            &raw[*e_shoff] as *const u8 as *const SectionHeader<T>,
            fh.e_shnum as usize,
          );
          let shstrtab = &shs[fh.e_shstrndx as usize];
          let sh_offset: &usize = transmute(&shstrtab.sh_offset);
          let sh_size: &usize = transmute(&shstrtab.sh_size);
          let shstr = &raw[*sh_offset..*sh_offset + *sh_size];
          Ok(ELFFile::<T> {
            raw,
            shstr,
            fh,
            phs,
            shs,
          })
        }
      } else {
        Err("magic number missmatch.")
      }
    }
  }
  pub fn getSectionName(&self, sh: &SectionHeader<T>) -> &str {
    let start = sh.sh_name as usize;
    let mut end = start;
    while self.shstr[end] != 0 {
      end += 1;
    }
    from_utf8(&self.shstr[start..end]).unwrap()
  }
}

#[derive(Debug)]
#[repr(C)]
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

#[derive(Debug)]
#[repr(C)]
pub struct ProgramHeader<T> {
  pub p_type: T,
  pub p_offset: T,
  pub p_vaddr: T,
  pub p_paddr: T,
  pub p_filesz: T,
  pub p_memsz: u64,
  pub p_align: T,
}

#[derive(Debug)]
#[repr(C)]
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

#[derive(Debug)]
#[repr(C)]
pub struct E_IDENT {
  EI_MAG: [u8; 4],
  pub EI_CLASS: u8,
  pub EI_DATA: u8,
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

pub const MAGIC: &[u8] = "\x7fELF".as_bytes();
