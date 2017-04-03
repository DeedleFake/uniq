use std::iter::{FromIterator};
use std::env::{args};
use std::collections::{HashSet};
use std::io::{stdin, BufRead};

struct Flags {
	bad: Option<String>,

	help: bool,
}

impl FromIterator<String> for Flags {
	fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=String> {
		let mut flags = Flags{
			bad: None,

			help: false,
		};

		for arg in iter {
			match arg.as_ref() {
				"-h" | "--help" => flags.help = true,
				_ => {
					if arg.chars().next().unwrap() == '-' {
						flags.bad = Some(arg);
						return flags;
					}
				},
			}
		}

		flags
	}
}

fn main() {
	let flags = args().collect::<Flags>();
	if let Some(ref bad) = flags.bad {
		println!("Unknown flag: {}", bad);
	}
	if flags.help || flags.bad.is_some() {
		println!("Usage: {}", args().next().unwrap());
	}

	let mut lines = HashSet::new();

	let stdin = stdin();
	for line in stdin.lock().lines() {
		match line {
			Ok(line) => {
				if lines.insert(line.clone()) {
					println!("{}", line);
				}
			},

			Err(err) => panic!(err),
		}
	}
}
