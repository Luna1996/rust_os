use alloc::{slice::from_raw_parts_mut, vec::Vec};
use uefi::services::{
  boot::{EFI_MEMORY_DESCRIPTOR, EFI_MEMORY_TYPE::*},
  runtime::EFI_RESET_TYPE,
};

//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
pub unsafe fn init() {
  let boot = (*crate::ST).BootServices;
  let mut memory_map_size = 0;
  let mut map_key = 0;
  let mut desc_size = 0;
  let mut desc_ver = 0;
  (boot.GetMemoryMap)(
    &mut memory_map_size,
    &mut *core::ptr::null_mut(),
    &mut map_key,
    &mut desc_size,
    &mut desc_ver,
  );
  memory_map_size += desc_size;
  let num_desc = memory_map_size / desc_size;
  let mut memory_map = Vec::<EFI_MEMORY_DESCRIPTOR>::with_capacity(num_desc);
  memory_map.set_len(num_desc);
  (boot.GetMemoryMap)(
    &mut memory_map_size,
    memory_map.as_mut_ptr(),
    &mut map_key,
    &mut desc_size,
    &mut desc_ver,
  );
  let err = (boot.ExitBootServices)(crate::HIMG, map_key);
  for desc in &mut memory_map {
    desc.VirtualStart = desc.PhysicalStart;
  }
  let runtime = (*crate::ST).RuntimeServices;
  (runtime.SetVirtualAddressMap)(memory_map_size, desc_size, desc_ver, memory_map.as_ptr());
  if err == 0 {
    let protocol = &*crate::VP;
    let video_buffer_base = protocol.Mode.FrameBufferBase;
    let video_buffer_size = protocol.Mode.FrameBufferSize / 4;
    let video_buffer = from_raw_parts_mut(
      protocol.Mode.FrameBufferBase as *mut u32,
      protocol.Mode.FrameBufferSize / 4,
    );
    let w = protocol.Mode.Info.HorizontalResolution as usize;
    let h = protocol.Mode.Info.VerticalResolution as usize;
    let d = 0;
    let c = 0x00FF00;
    for x in d..w - d - 1 {
      video_buffer[x + d * w] = c;
      video_buffer[x + (h - d - 1) * w] = c;
    }
    for y in d..h - d - 1 {
      video_buffer[d + y * w] = c;
      video_buffer[w - d - 1 + y * w] = c;
    }
  }

  // ((*crate::ST).RuntimeServices.ResetSystem)(EFI_RESET_TYPE::EfiResetCold, 0, 0, core::ptr::null());
}
