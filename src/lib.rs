//! Naive implementation of path swap.

extern crate libc;

mod platform;

use std::io;
use std::path::Path;

/// Swaps the content of paths `a` and `b`.
pub fn swap<A, B>(a: A, b: B) -> io::Result<()> where A: AsRef<Path>, B: AsRef<Path> {
	platform::swap(a, b)
}

#[cfg(test)]
mod tests {
	extern crate tempdir;
	use std::fs;
	use std::path::Path;
	use std::io::{Write, Read};
	use self::tempdir::TempDir;
	use super::swap;

	fn write_to_file<P: AsRef<Path>>(file: P, text: &str) {
		let mut file = fs::OpenOptions::new()
			.create(true)
			.write(true)
			.open(file)
			.unwrap();
		file.write_all(text.as_ref()).unwrap();
		file.flush().unwrap();
	}

	fn read_from_file<P: AsRef<Path>>(file: P) -> String {
		let mut buffer = String::new();
		let mut file = fs::OpenOptions::new()
			.read(true)
			.open(file)
			.unwrap();
		file.read_to_string(&mut buffer).unwrap();
		buffer
	}

	#[test]
	fn test_swap_files() {
		let dir = TempDir::new("").unwrap();
		let path_a = dir.path().join("file_a");
		let path_b = dir.path().join("file_b");
		write_to_file(&path_a, "foo");
		write_to_file(&path_b, "bar");
		swap(&path_a, &path_b).unwrap();
		let read_a = read_from_file(&path_a);
		let read_b = read_from_file(&path_b);
		assert_eq!("bar", read_a);
		assert_eq!("foo", read_b);
	}

	#[test]
	fn test_swap_dirs() {
		let dir_a = TempDir::new("a").unwrap();
		let dir_b = TempDir::new("b").unwrap();
		let path_a = dir_a.path().join("file");
		let path_b = dir_b.path().join("file");
		write_to_file(&path_a, "foo");
		write_to_file(&path_b, "bar");
		swap(&dir_a, &dir_b).unwrap();
		let read_a = read_from_file(&path_a);
		let read_b = read_from_file(&path_b);
		assert_eq!("bar", read_a);
		assert_eq!("foo", read_b);
	}
}
