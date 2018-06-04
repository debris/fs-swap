use std::os::unix::ffi::OsStrExt;
use std::{io, ffi};
use std::path::Path;
use libc;

extern "stdcall" {
	fn renamex_np(oldpath: *const libc::c_char, newpath: *const libc::c_char, flags: libc::c_uint) -> libc::c_int;
}

pub fn swap<A, B>(a: A, b: B) -> io::Result<()> where A: AsRef<Path>, B: AsRef<Path> {
	const RENAME_SWAP: libc::c_uint = 2;

	let a_path = ffi::CString::new(a.as_ref().as_os_str().as_bytes())
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
	let b_path = ffi::CString::new(b.as_ref().as_os_str().as_bytes())
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

	match unsafe { renamex_np(a_path.as_ptr(), b_path.as_ptr(), RENAME_SWAP) } {
		0 => Ok(()),
		code => Err(io::Error::new(io::ErrorKind::Other, format!("renamex_np failed with code: {}", code))),
	}
}
