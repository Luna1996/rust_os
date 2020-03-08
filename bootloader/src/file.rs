use crate::s16::S16;
use alloc::{string::String, vec::Vec};
use core::{mem::transmute, ptr::null_mut};
use uefi::{
  guid::{EFI_FILE_INFO_ID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID},
  protocols::media_access::{
    EFI_FILE_INFO, EFI_FILE_MODE_READ, EFI_FILE_PROTOCOL, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
  },
  services::boot::EFI_LOCATE_SEARCH_TYPE::ByProtocol,
  types::EFI_HANDLE,
};

//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
pub unsafe fn init<'a>() -> &'a EFI_FILE_PROTOCOL {
  let st = &*crate::ST;
  let fs_protocols = {
    let mut handels_buf_size: usize = 0;
    (st.BootServices.LocateHandle)(
      ByProtocol,
      &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
      0 as *mut u8,
      &mut handels_buf_size,
      &mut (0 as *mut u8),
    );
    let num_of_protocols = handels_buf_size / 8;
    let mut handles: Vec<EFI_HANDLE> = Vec::with_capacity(num_of_protocols);
    let mut fs_protocols: Vec<&EFI_SIMPLE_FILE_SYSTEM_PROTOCOL> =
      Vec::with_capacity(num_of_protocols);
    handles.set_len(num_of_protocols);
    fs_protocols.set_len(num_of_protocols);
    (st.BootServices.LocateHandle)(
      ByProtocol,
      &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
      0 as *mut u8,
      &mut handels_buf_size,
      &mut handles[0],
    );
    for i in 0..num_of_protocols {
      (st.BootServices.HandleProtocol)(
        handles[i],
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        transmute(&mut fs_protocols[i]),
      );
    }
    fs_protocols
  };
  let mut root = &*null_mut();
  (fs_protocols[0].OpenVolume)(fs_protocols[0], &mut root);
  root
}

//╔═══════════╗
//║  ReadFile ║
//╚═══════════╝
pub unsafe fn read_file(root: &EFI_FILE_PROTOCOL, path: &str) -> Vec<u8> {
  let mut fp = &*null_mut();
  let path = S16::from(String::from(path));
  (root.Open)(root, &mut fp, path.0.as_ptr(), EFI_FILE_MODE_READ, 0);
  let mut info_buf_size: usize = 0;
  (fp.GetInfo)(fp, &EFI_FILE_INFO_ID, &mut info_buf_size, 0 as *mut u8);
  let mut info_buf = vec![0 as u8; info_buf_size];
  (fp.GetInfo)(
    fp,
    &EFI_FILE_INFO_ID,
    &mut info_buf_size,
    info_buf.as_mut_ptr(),
  );
  let info: &EFI_FILE_INFO = transmute(info_buf.as_ptr());
  let mut file_buf_size = info.FileSize as usize;
  let mut file_buf = vec![0 as u8; file_buf_size];
  (fp.Read)(fp, &mut file_buf_size, file_buf.as_mut_ptr());
  file_buf
}

//╔═══════════╗
//║  LoadELF  ║
//╚═══════════╝
