//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
use core::{mem::transmute, slice::from_raw_parts_mut};
use uefi::{
  efi_system_table::EFI_SYSTEM_TABLE, protocols::console_support::EFI_GRAPHICS_OUTPUT_PROTOCOL,
  services::boot::EFI_LOCATE_SEARCH_TYPE::ByProtocol, types::EFI_HANDLE,
};

pub unsafe fn init(st: &EFI_SYSTEM_TABLE) {
  let video_protocol = {
    let guid = uefi::guid::EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    let mut buf_size: usize = 0;
    (st.BootServices.LocateHandle)(
      ByProtocol,
      &guid,
      0 as *mut u8,
      &mut buf_size,
      &mut (0 as *mut u8),
    );
    let buf = efi_alloc!(EFI_HANDLE, buf_size / 8);
    (st.BootServices.LocateHandle)(ByProtocol, &guid, 0 as *mut u8, &mut buf_size, &mut buf[0]);
    let mut g = efi_alloc!(EFI_GRAPHICS_OUTPUT_PROTOCOL);
    (st.BootServices.HandleProtocol)(buf[0], &guid, transmute(&mut g));
    g
  };
  let video_buffer = from_raw_parts_mut(
    video_protocol.Mode.FrameBufferBase as *mut u32,
    video_protocol.Mode.FrameBufferSize / 4,
  );
  let w = video_protocol.Mode.Info.HorizontalResolution as usize;
  let h = video_protocol.Mode.Info.VerticalResolution as usize;
  for i in 0..100 {
    video_buffer[i + i * w] = 0xFF0000;
  }
  efi_print!(
    "({}x{})({})(0x{:0>16X})\r\n",
    video_protocol.Mode.Info.HorizontalResolution,
    video_protocol.Mode.Info.VerticalResolution,
    video_protocol.Mode.FrameBufferSize,
    video_protocol.Mode.FrameBufferBase
  );
}
