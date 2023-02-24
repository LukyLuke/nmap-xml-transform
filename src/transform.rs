
use crate::types::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde_xml_rs::from_str;
use minijinja::{Environment, value::Value};

pub fn from_xml(nmap_file: &str, template: &str) -> Result<(), Box<dyn std::error::Error>> {
	let parsed = parse_nmap_xml(nmap_file)?;
	let source = read_template_string(template)?;

	let mut env = Environment::new();
	env.add_template("nmap", &source)?;
	let context = Value::from_struct_object(parsed);
	let tpl = env.get_template("nmap")?;
	let rendered = tpl.render(context)?;

	print!("{}\n", rendered);

	Ok(())
}

fn parse_nmap_xml(nmap_file: &str) -> Result<NmapRun, Box<dyn std::error::Error>> {
	let path = Path::new(nmap_file);
	let display = path.display();

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => return Err(Box::from(format!("Failed to open {}: {:#}", display, err)))
	};

	let mut content = String::new();
	match file.read_to_string(&mut content) {
		Err(err) => return Err(Box::from(format!("Failed to read {}: {:#}", display, err))),
		_ => {}
	}

	match from_str::<NmapRun>(&content) {
		Ok(parsed) => Ok(parsed),
		Err(err) => Err(Box::from(format!("Failed to parse {}: {:#}", display, err)))
	}
}

fn read_template_string(template_file: &str) -> Result<String, Box<dyn std::error::Error>> {
	let path = Path::new(template_file);
	let display = path.display();

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => return Err(Box::from(format!("Failed to open {}: {:#}", display, err)))
	};

	let mut content = String::new();
	match file.read_to_string(&mut content) {
		Err(err) => return Err(Box::from(format!("Failed to read {}: {:#}", display, err))),
		_ => Ok(content)
	}
}
