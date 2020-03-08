use alloc::vec::Vec;
use core::mem::transmute;
use uefi::{
  guid::EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
  protocols::console_support::EFI_GRAPHICS_OUTPUT_PROTOCOL,
  services::boot::EFI_LOCATE_SEARCH_TYPE::ByProtocol, types::EFI_HANDLE,
};

//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
pub unsafe fn init() {
  let st = &*crate::ST;
  let protocols = {
    let mut handels_buf_size: usize = 0;
    (st.BootServices.LocateHandle)(
      ByProtocol,
      &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
      0 as *mut u8,
      &mut handels_buf_size,
      &mut (0 as *mut u8),
    );
    let num_of_handels = handels_buf_size / 8;
    let mut handles: Vec<EFI_HANDLE> = Vec::with_capacity(num_of_handels);
    let mut protocols: Vec<&EFI_GRAPHICS_OUTPUT_PROTOCOL> = Vec::with_capacity(num_of_handels);
    handles.set_len(num_of_handels);
    (st.BootServices.LocateHandle)(
      ByProtocol,
      &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
      0 as *mut u8,
      &mut handels_buf_size,
      &mut handles[0],
    );
    protocols.set_len(num_of_handels);
    for i in 0..num_of_handels {
      (st.BootServices.HandleProtocol)(
        handles[i],
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        transmute(&mut protocols[i]),
      );
    }
    protocols
  };
  crate::VP = protocols[0];
}
