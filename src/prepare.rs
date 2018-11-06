use std::{env, fs, path::PathBuf};

use crate::common::*;

use failure::*;
type Result<T> = ::std::result::Result<T, Error>;

pub fn main(root: Option<&str>) -> Result<PathBuf> {
	find_project()?;
	crate::compile::main()?;

	let path = root.map_or_else(|| get_ns2(), |root| PathBuf::from(root).join("x64"));

	#[cfg(not(windows))]
	use std::os::unix::fs::symlink;
	#[cfg(windows)]
	use std::os::windows::fs::symlink_dir as symlink;

	let mod_dir = &path.join("../laspad_mod");
	if mod_dir.exists() {
		fs::remove_file(mod_dir)?
	};
	let compiled = &env::current_dir()?.join("compiled");
	symlink(compiled, mod_dir)?;

	Ok(path)
}
