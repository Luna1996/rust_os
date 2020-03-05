#[macro_export]
macro_rules! efi_print {
  ($($arg:tt)*) => {{
    let cout = $crate::ST.unwrap().ConOut;
    (cout.OutputString)(cout, $crate::console::s16(&format!($($arg)*)[..]));
  }};
}

#[macro_export]
macro_rules! efi_alloc {
  ($T:ty) => {{
    let mut buf: *mut u8 = core::ptr::null_mut();
    ($crate::ST.unwrap().BootServices.AllocatePool)(
      uefi::services::boot::EFI_MEMORY_TYPE::EfiLoaderData,
      core::mem::size_of::<$T>(),
      &mut buf,
    );
    &mut *(buf as *mut $T)
  }};
  ($T:ty, $s:expr) => {{
    let mut buf: *mut u8 = core::ptr::null_mut();
    ($crate::ST.unwrap().BootServices.AllocatePool)(
      uefi::services::boot::EFI_MEMORY_TYPE::EfiLoaderData,
      $s * core::mem::size_of::<$T>(),
      &mut buf,
    );
    core::slice::from_raw_parts_mut(buf as *mut $T, $s)
  }};
}
