
use crate::types::*;
use serde_xml_rs::from_str;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn from_xml(file: &str) -> Result<(), String> {
	let path = Path::new(file);
	let display = path.display();

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => return Err(format!("Failed to open {}: {:#}", display, err))
	};

	let mut content = String::new();
	match file.read_to_string(&mut content) {
		Err(err) => return Err(format!("Failed to read {}: {:#}", display, err)),
		_ => {}
	}

	let parsed = match from_str::<NampRun>(&content) {
		Ok(nmap) => nmap,
		Err(err) => return Err(format!("Failed to parse {}: {:#}", display, err))
	};

	log::error!("Parsed: {:?}", parsed);

	Ok(())
}
