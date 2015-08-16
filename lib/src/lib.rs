#![feature(associated_consts)]

pub trait FromIndex{
	type Index;

	fn from_index(index: Self::Index) -> Option<Self>;
}

pub trait ToIndex{
	type Index;

	fn into_index(self) -> Self::Index;
	fn index(&self) -> Self::Index;
}

pub trait Len{
	const LEN: usize;
}

pub trait Ends{
	fn first() -> Self;
	fn last() -> Self;
}
