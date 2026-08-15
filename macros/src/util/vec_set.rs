use alloc::vec::Vec;
use core::iter::Iterator;

pub struct VecSet<T>(Vec<T>);

impl<T: Copy + Eq> VecSet<T>{
	pub fn from_vec(v: Vec<T>) -> Self{
		VecSet(v)
	}

	pub fn contains(&self,x: T) -> bool{
		self.0.iter().cloned().any(|y| x == y)
	}

	pub fn insert(&mut self,x: T){
		if !self.contains(x){
			self.0.push(x);
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &T>{
		self.0.iter()
	}
}
