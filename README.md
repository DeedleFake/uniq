uniq
====

A Rust crate that extends Iterators to make it easier to skip duplicates. I wrote this just for the heck of it. I don't intend for it to ever be used.

Example
-------

```
use std::io::{stdin, BufRead};
use std::fmt::{Display};

extern crate uniq;
use uniq::{UniqIterator};

fn panic<T, E>(e: Result<T, E>) -> T where E: Display {
	match e {
		Ok(e) => e,
		Err(err) => panic!("{}", err),
	}
}

fn main() {
	let stdin = stdin();
	for line in stdin.lock().lines().map(panic).uniq() {
		println!("{}", line);
	}
}
```
