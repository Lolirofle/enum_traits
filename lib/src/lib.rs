//! Simple traits for builtin enum items.
//! Primarily used by `enum_traits_macros` when automatically deriving types.
//! The crate `enum_traits_macros` is required for the derives.

#![feature(associated_consts)]

///Represents the type used for indexing the variants of the enum item.
///`Type` should be an primitive integer type and have more values or an equal number of values compared to the number of variants in the enum item.
///
///Derive this trait for an enum automatically using `#[derive(EnumIndex)]`
///When derived, `Type` becomes the type specified in the `repr` attribute for the enum item.
///If a `repr` attribute does not exist, a calculated minimum integer type based on the number of variant fields is used instead.
pub trait Index{
	///Type used as an index for the enum
	type Type;
}

///Derive this trait for an enum automatically using `#[derive(EnumFromIndex)]`
pub trait FromIndex: Index + Sized{
	///Tries to construct `Self` from an index based on the variants' defined order
	fn from_index(index: <Self as Index>::Type) -> Option<Self>;

	///Constructs `Self` from an index based on the variants' defined order
	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self;
}

///Derive this trait for an enum automatically using `#[derive(EnumToIndex)]`
pub trait ToIndex: Index{
	///Index in the defined order of an enum
	fn into_index(self) -> <Self as Index>::Type;

	///Index in the defined order of an enum
	fn index(&self) -> <Self as Index>::Type;
}

///Derive this trait for an enum automatically using `#[derive(EnumLen)]`
pub trait Len{
	///Number of variants in an enum
	const LEN: usize;
}

///Derive this trait for an enum automatically using `#[derive(EnumEnds)]`
pub trait Ends: Sized{
	///The first variant in the defined order of an enum
	fn first() -> Self;

	///The last variant in the defined order of an enum
	fn last() -> Self;
}

///Derive this trait for an enum automatically using `#[derive(EnumDiscriminant)]`
///This trait can only be derived when every item have an explicitly defined discriminant.
pub trait Discriminant: Sized{
	///The type of the discriminant
	type Type;

	///Tries to construct an enum from the discriminant of the variants/enum items
	fn from_discriminant(discriminant: <Self as Discriminant>::Type) -> Option<Self>;

	///Constructs an enum from the discriminant of the variants/enum items
	unsafe fn from_discriminant_unchecked(discriminant: <Self as Discriminant>::Type) -> Self;
}

///Derive this trait for an enum automatically using `#[derive(EnumIter)]`
pub trait Iterable{
	///The type of the iterator
	type Iter: Iterator;

	///Constructs an iterator that iterates over every variant in the defined order
	fn variants() -> Self::Iter;
}
