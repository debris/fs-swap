use std::os::unix::ffi::OsStrExt;
use std::{io, ffi};
use std::path::Path;
use libc;

extern "stdcall" {
	fn renameat2(
		olddirfd: libc::c_int, oldpath: *const libc::c_char, 
		newdirfd: libc::c_int, newpath: *const libc::c_char, flags: libc::c_int
	) -> libc::c_int;
}

pub fn swap<A, B>(a: A, b: B) -> io::Result<()> where A: AsRef<Path>, B: AsRef<Path> {
	let a_path = ffi::CString::new(a.as_ref().as_os_str().as_bytes())
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
	let b_path = ffi::CString::new(b.as_ref().as_os_str().as_bytes())
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

	match unsafe { renameat2(libc::AT_FDCWD, a_path.as_ptr(), libc::AT_FDCWD, b_path.as_ptr(), libc::RENAME_EXCHANGE) } {
		0 => Ok(()),
		code => Err(io::Error::new(io::ErrorKind::Other, format!("renameat2 failed with code: {}", code))),
	}
}
