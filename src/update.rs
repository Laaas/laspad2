use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use curl::easy::Easy;
use failure::*;
use serde_xml_rs;
use std::{
	fs::{self, File},
	io::{Cursor, Read, Write},
	path::Path,
};
use zip::read::ZipArchive;

use crate::{common, steam::Item};

type Result<T> = ::std::result::Result<T, Error>;

mod ns2_xml_format {
	use serde_derive::Deserialize;

	#[derive(Deserialize, Debug)]
	pub struct PublishedFile {
		pub publishedfileid: u64,
		pub file_url:        Box<str>,
		pub time_updated:    u64,
	}

	#[derive(Deserialize, Debug)]
	pub struct PublishedFileDetails {
		pub publishedfile: PublishedFile,
	}

	#[derive(Deserialize, Debug)]
	pub struct Response {
		pub publishedfiledetails: PublishedFileDetails,
	}

	#[derive(Deserialize, Debug)]
	pub struct Root {
		pub response: Response,
	}
}

use self::ns2_xml_format::Response as NS2XMLFormat;

fn download(url: &str) -> Result<Vec<u8>> {
	let mut buf = Vec::new();
	let mut easy = Easy::new();
	easy.url(url)?;
	{
		let mut transfer = easy.transfer();
		transfer.write_function(|data| {
			buf.extend_from_slice(data);
			Ok(data.len())
		})?;
		transfer.perform()?;
	}
	Ok(buf)
}

pub fn specific<P: AsRef<Path>>(item: Item, path: P) -> Result<()> {
	let path = path.as_ref();

	let format: NS2XMLFormat = serde_xml_rs::deserialize(&*download(&format!(
		"http://mods.ns2cdt.com/ISteamRemoteStorage/GetPublishedFileDetails/V0001?format=xml&publishedfileid={}",
		item.0
	))?).with_context(|_| format!("Could not deserialize XML from Steam for {:X}", item.0))?;

	let local_update = {
		let path = path.join(".update_timestamp");
		if path.exists() {
			File::open(&path)?.read_u64::<LE>()?
		} else {
			0
		}
	};

	let remote_update = format.publishedfiledetails.publishedfile.time_updated;
	if local_update < remote_update {
		log!(1; "Local workshop item {:8X} copy is outdated, {} < {}", item.0, local_update, remote_update);
		for entry in fs::read_dir(path)? {
			let entry = &entry?.path();
			if entry
				.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.chars()
				.next()
				.unwrap() != '.'
			{
				if entry.is_dir() {
					fs::remove_dir_all(entry)?;
				} else {
					fs::remove_file(entry)?;
				};
			};
		}

		let url = &format.publishedfiledetails.publishedfile.file_url;
		let buf = download(url)?;
		let mut archive = ZipArchive::new(Cursor::new(buf))
			.with_context(|_| format!("Could not read zip archive for {:X} @ {}", item.0, url))?;
		for i in 0..archive.len() {
			let mut file = archive.by_index(i).with_context(|_| {
				format!("Could not access file in zip archive for {:X}", item.0)
			})?;
			let path = path.join(file.name());
			fs::create_dir_all(path.parent().unwrap())?;
			let mut buf = Vec::new();
			file.read_to_end(&mut buf)?;
			File::create(path)?.write_all(&buf)?;
		}
		File::create(path.join(".update_timestamp"))?.write_u64::<LE>(remote_update)?;
	} else {
		log!(1; "Local workshop item {:8X} copy is up-to-date", item.0);
	};

	Ok(())
}

pub fn main() -> Result<()> {
	common::find_project()?;

	let dependencies = Path::new("dependencies");
	if dependencies.exists() {
		for dep in fs::read_dir(dependencies)? {
			let path = &dep?.path();
			if let Ok(modid) = u64::from_str_radix(path.file_name().unwrap().to_str().unwrap(), 16)
			{
				if let Err(e) = specific(Item(modid), path) {
					elog!("Could not update {:X}: {}", modid, e);
				};
			};
		}
	};

	log!("Finished updating");
	Ok(())
}
