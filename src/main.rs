
mod types;
mod transform;

use std::env;
use transform::from_xml;

pub use log::{info, warn, debug};

fn main() ->Result<(), Box<dyn std::error::Error>> {
	env_logger::init();
	let args: Vec<String> = env::args().collect();
	let usage = "Usage: nmap_xml (INPUT TEMPLATE | --format | -f)\n       INPUT\t\tXML-File from -oX nmap command\n       TEMPLATE\t\tJinja-2 Template file\n       --format|-f\tShows the data format to use in the output template".to_string();
	let special = "Special:\n  finish_time\n  elapsed_time\n  status\n  runstats::hosts_total\n  runstats::hosts_online\n  runstats::hosts_offline\n  hosts::host::ipv4\n  hosts::host::ipv6\n  hosts::host::mac\n  hosts::host::os::vendor\n  hosts::host::os::purpose";
	let comment = "The Datastructure is a 1:1 mapping from nmap xml structure";

	match args.len() {
		f if f == 2 && (args[f - 1] == "--format" || args[f - 1] == "-f") => {
			println!("{}\n\nData-Structure: {}\n{:#?}\n\n{}", usage, comment, types::NmapRun::empty(), special);
			Ok(())
		},
		err if err < 3 => {
			println!("{}", usage);
			Ok(())
		},
		_ => {
			from_xml(&args[args.len() - 2], &args[args.len() - 1])
		},
	}

}
