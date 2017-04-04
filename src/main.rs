use std::iter::{FromIterator};
use std::env::{args};
use std::collections::{HashSet};
use std::collections::hash_map::{RandomState};
use std::io::{stdin, BufRead};
use std::hash::{Hash};
use std::fmt::{Display};

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

struct Uniq<T, I> where T: Iterator<Item=I>, I: Hash + Eq + Clone {
	set: HashSet<I, RandomState>,
	iter: T,
}

impl<T, I> Iterator for Uniq<T, I> where T: Iterator<Item=I>, I: Hash + Eq + Clone {
	type Item = I;

	fn next(&mut self) -> Option<Self::Item> {
		match self.iter.next() {
			Some(item) => {
				if self.set.insert(item.clone()) {
					return Some(item);
				}
				return self.next();
			},

			None => None,
		}
	}
}

trait UniqIterator<T, I> where T: Iterator<Item=I>, I: Hash + Eq + Clone {
	fn uniq(self) -> Uniq<T, I>;
}

impl<T, I> UniqIterator<T, I> for T where T: Iterator<Item=I>, I: Hash + Eq + Clone {
	fn uniq(self) -> Uniq<T, I> {
		Uniq{
			set: HashSet::new(),
			iter: self,
		}
	}
}

fn panic<T, E>(e: Result<T, E>) -> T where E: Display {
	match e {
		Ok(t) => t,
		Err(err) => panic!("{}", err),
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

	let stdin = stdin();
	for line in stdin.lock().lines().map(panic).uniq() {
		println!("{}", line);
	}
}
