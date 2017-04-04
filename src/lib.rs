use std::collections::{HashSet};
use std::collections::hash_map::{RandomState};
use std::hash::{Hash};

pub struct Uniq<T, I> where T: Iterator<Item=I>, I: Hash + Eq + Clone {
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

pub trait UniqIterator<T, I> where T: Iterator<Item=I>, I: Hash + Eq + Clone {
	fn uniq(self) -> Uniq<T, I>;
}

impl<N, T, I> UniqIterator<T, I> for N where N: IntoIterator<Item=I, IntoIter=T>, T: Iterator<Item=I>, I: Hash + Eq + Clone {
	fn uniq(self) -> Uniq<T, I> {
		Uniq{
			set: HashSet::new(),
			iter: self.into_iter(),
		}
	}
}
