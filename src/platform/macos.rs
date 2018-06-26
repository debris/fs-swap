extern crate libc;

use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{io, ffi};

extern "stdcall" {
	fn exchangedata(oldpath: *const libc::c_char, newpath: *const libc::c_char, flags: libc::c_uint) -> libc::c_int;
}

pub fn swap<A, B>(a: A, b: B) -> io::Result<()> where A: AsRef<Path>, B: AsRef<Path> {
	let a_path = ffi::CString::new(a.as_ref().as_os_str().as_bytes())?;
	let b_path = ffi::CString::new(b.as_ref().as_os_str().as_bytes())?;

	unsafe {
		// `swap` files using swapping directories
		// <https://www.unix.com/man-page/osx/2/exchangedata/>
		if exchangedata(a_path.as_ptr(), b_path.as_ptr(), 0) == 0 {
			Ok(())
		} else {
			Err(io::Error::new(io::ErrorKind::Other, format!("exchangedata failed with code: {}", *libc::__error())))
		}
	}
}
