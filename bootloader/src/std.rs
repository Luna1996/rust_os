use alloc::alloc::{GlobalAlloc, Layout};
// use alloc::vec::Vec;
use core::ptr::null_mut;
use core::slice::from_raw_parts_mut;
use core::str::from_utf8_unchecked;
use uefi::efi_system_table::{EFI_BOOT_SERVICES, EFI_SYSTEM_TABLE};
use uefi::services::boot::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE};

pub static mut ST: Option<&EFI_SYSTEM_TABLE> = None;
pub static mut BOOT: Option<&EFI_BOOT_SERVICES> = None;
// pub static mut BUFFERS: Vec<*mut u8> = Vec::new();

#[macro_export]
macro_rules! efi_print {
  ($($arg:tt)*) => {{
    let cout = $crate::std::ST.unwrap().ConOut;
    (cout.OutputString)(cout, $crate::s16(&format!($($arg)*)[..]));
  }};
}

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
  loop {}
}

#[global_allocator]
static ALLOCATOR: _Allocator = _Allocator {};
struct _Allocator {}
unsafe impl GlobalAlloc for _Allocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let n = layout.size();
    let a = layout.align();
    let mut buf: *mut u8 = null_mut();
    if a < 4096 {
      (BOOT.unwrap().AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n, &mut buf);
    } else {
      (BOOT.unwrap().AllocatePages)(
        EFI_ALLOCATE_TYPE::AllocateAnyPages,
        EFI_MEMORY_TYPE::EfiLoaderData,
        n >> 12,
        &mut (buf as u64),
      );
    }
    buf
  }
  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    let n = layout.size();
    let a = layout.align();
    if a < 4096 {
      (BOOT.unwrap().FreePool)(ptr);
    } else {
      (BOOT.unwrap().FreePages)(ptr as u64, n >> 12);
    }
  }
}

pub unsafe fn s16(s8: &str) -> *const u16 {
  let n = s8.len() + 1;
  let mut buf8: *mut u8 = null_mut();
  (BOOT.unwrap().AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n << 1, &mut buf8);
  // BUFFERS.push(buf8);
  let buf = from_raw_parts_mut(buf8 as *mut u16, n);
  buf[n - 1] = 0;
  for (i, c) in s8.encode_utf16().enumerate() {
    buf[i] = c;
  }
  &buf[0]
}

pub unsafe fn s8(s16: &*const u16) -> &str {
  let n = {
    let mut n = 0;
    while *(s16.add(n)) != 0 {
      n = n + 1;
    }
    n
  };
  let buf = {
    let mut buf: *mut u8 = null_mut();
    (BOOT.unwrap().AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n, &mut buf);
    from_raw_parts_mut(buf as *mut u8, n)
  };
  // BUFFERS.push(buf8);
  for i in 0..n {
    buf[i] = *(s16.add(i)) as u8;
  }
  from_utf8_unchecked(buf)
}

pub unsafe fn init(st: &'static EFI_SYSTEM_TABLE) {
  ST = Some(st);
  BOOT = Some(st.BootServices);
}
