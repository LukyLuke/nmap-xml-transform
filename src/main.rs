
mod types;
mod transform;

use std::env;
use transform::from_xml;
use env_logger;

pub use log::{info, warn, debug};

fn main() -> Result<(), String> {
	env_logger::init();
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 | 3 => from_xml(&args[args.len() - 1]),
		_ => Err("Usage: nmap_xml [--file] nmap_output.xml".to_string()),
	}

}
