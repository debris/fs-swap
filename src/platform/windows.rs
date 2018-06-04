use std::io;
use std::path::Path;

pub fn swap<A, B>(a: A, b: B) -> io::Result<()> where A: AsRef<Path>, B: AsRef<Path> {
	unimplemented!();
}
