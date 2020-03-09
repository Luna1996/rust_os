#![no_std]
#![no_main]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![feature(const_raw_ptr_deref, alloc_error_handler)]

#[macro_use]
extern crate alloc;

use alloc::{
	alloc::{GlobalAlloc, Layout},
	slice::from_raw_parts_mut,
	string::String,
	vec::Vec,
};
use core::{
	mem::transmute,
	panic::PanicInfo,
	ptr::{null, null_mut},
};
use elf::ELFFile;
use uefi::{
	efi_system_table::{EFI_BOOT_SERVICES, EFI_SYSTEM_TABLE},
	guid::{
		EFI_FILE_INFO_ID, EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
	},
	protocols::{
		console_support::{EFI_GRAPHICS_OUTPUT_PROTOCOL, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL},
		media_access::{EFI_FILE_INFO, EFI_FILE_MODE_READ, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL},
	},
	services::boot::{
		EFI_ALLOCATE_TYPE, EFI_LOCATE_SEARCH_TYPE::ByProtocol, EFI_MEMORY_DESCRIPTOR, EFI_MEMORY_TYPE,
	},
	types::{EFI_HANDLE, EFI_STATUS},
};

//╔════════════╗
//║ 	Globle	 ║
//╚════════════╝
pub const KERNEL: &str = "kernel";
pub static mut BOOT: *const EFI_BOOT_SERVICES = null();
pub static mut COUT: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL = null();

//╔═══════════╗
//║ 	Entry		║
//╚═══════════╝
#[no_mangle]
unsafe fn efi_main(img: EFI_HANDLE, st: &'static EFI_SYSTEM_TABLE) -> EFI_STATUS {
	// Setup globels
	let boot = st.BootServices;
	let cout = st.ConOut;
	BOOT = boot;
	COUT = cout;
	// Setup VGA Text for boottime.
	(cout.ClearScreen)(cout);
	(cout.SetMode)(cout, (cout.Mode.MaxMode - 1) as usize);
	// Setup VGA for boottime, locate VGA buffer base for runtime.
	let gop = {
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
		protocols[0]
	};
	// Setup fs for boottime.
	let fsp = {
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
		let mut root = &*null_mut();
		(fs_protocols[0].OpenVolume)(fs_protocols[0], &mut root);
		root
	};
	// Open os kernel entry file.
	let file_buf = {
		let mut fp = &*null_mut();
		let path = S16::from(String::from(KERNEL));
		(fsp.Open)(fsp, &mut fp, path.0.as_ptr(), EFI_FILE_MODE_READ, 0);
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
	};
	// Parse and load the kernel file.
	let kernel_file = ELFFile::<u64>::parse(file_buf.as_slice()).unwrap();
	// Retrive memory map, exit boot services.
	let boot = st.BootServices;
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
	let err = (boot.ExitBootServices)(img, map_key);
	// Do something(draw a green frame) indicate exit boot services successfully.
	if err == 0 {
		let video_buffer_base = gop.Mode.FrameBufferBase as *mut u32;
		let video_buffer_size = gop.Mode.FrameBufferSize / 4;
		let video_buffer = from_raw_parts_mut(video_buffer_base, video_buffer_size);
		let w = gop.Mode.Info.HorizontalResolution as usize;
		let h = gop.Mode.Info.VerticalResolution as usize;
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
	loop {}
}

//╔═══════════╗
//║ 	Panic 	║
//╚═══════════╝
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
	print!("{}", info);
	loop {}
}

//╔═══════════╗
//║ Allocator ║
//╚═══════════╝
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
			((*BOOT).AllocatePool)(EFI_MEMORY_TYPE::EfiLoaderData, n, &mut buf);
		} else {
			((*BOOT).AllocatePages)(
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
			((*BOOT).FreePool)(ptr);
		} else {
			((*BOOT).FreePages)(ptr as u64, n >> 12);
		}
	}
}

//╔══════════════╗
//║  VGA-String  ║
//╚══════════════╝
struct S16(pub Vec<u16>, pub String);

impl From<String> for S16 {
	fn from(s8: String) -> S16 {
		let mut s16 = Vec::with_capacity(s8.len() + 1);
		for c16 in s8.encode_utf16() {
			s16.push(c16);
		}
		s16.push(0);
		S16(s16, s8)
	}
}

impl From<*const u16> for S16 {
	fn from(ptr: *const u16) -> S16 {
		unsafe {
			let mut n = 0;
			while *(ptr.add(n)) != 0 {
				n += 1;
			}
			print!("{}", n);
			let s16 = Vec::from_raw_parts(ptr as *mut u16, n, n);
			let s8 = String::from_utf16(s16.as_slice()).unwrap();
			S16(s16, s8)
		}
	}
}

//╔═════════════╗
//║  efi-print  ║
//╚═════════════╝
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
		((*crate::COUT).OutputString)(&(*crate::COUT), $crate::S16::from(format!($($arg)*)).0.as_ptr());
	}}
}
