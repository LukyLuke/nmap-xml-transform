
mod types;
mod transform;

use std::env;
use transform::from_xml;
use env_logger;

pub use log::{info, warn, debug};

fn main() {
	env_logger::init();
	let args: Vec<String> = env::args().collect();

	match args.len() {
		err if err < 3 => Err(Box::from("Usage: nmap_xml nmap_output.xml transform_to.tpl".to_string())),
		_ => {
			from_xml(&args[args.len() - 2], &args[args.len() - 1])
		},
	}.unwrap()

}
