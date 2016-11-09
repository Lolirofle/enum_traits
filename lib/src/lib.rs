//! Simple traits for the builtin enums.
//! The crate `enum_traits_macros` is required for the derives

#![feature(associated_consts)]

///Represents the type specified in the `repr` attribute for the enum item.
///If a `repr` attribute does not exist, a calculated minimum integer type based on the number of variant fields is used instead.
///
///Derive this trait for an enum using `#[derive(EnumIndex)]`
pub trait Index{
	///Type used as an index for the enum
	type Type;
}

///Derive this trait for an enum using `#[derive(EnumFromIndex)]`
pub trait FromIndex: Index + Sized{
	///Tries to construct an enum from a index in the enum's variants' defined order
	fn from_index(index: <Self as Index>::Type) -> Option<Self>;
}

///Derive this trait for an enum using `#[derive(EnumToIndex)]`
pub trait ToIndex: Index{
	///Index in defined order in an enum
	fn into_index(self) -> <Self as Index>::Type;

	///Index in defined order in an enum
	fn index(&self) -> <Self as Index>::Type;
}

///Derive this trait for an enum using `#[derive(EnumLen)]`
pub trait Len{
	///Number of variants in an enum
	const LEN: usize;
}

///Derive this trait for an enum using `#[derive(EnumEnds)]`
pub trait Ends: Sized{
	///First variant in defined order in an enum
	fn first() -> Self;

	///Last variant in defined order in an enum
	fn last() -> Self;
}

///Derive this trait for an enum using `#[derive(EnumDiscriminant)]`
pub trait Discriminant: Sized{
	type Type;

	///Tries to construct an enum from the discriminant of the variants/enum items
	fn from_discriminant(index: <Self as FromDiscriminant>::Type) -> Option<Self>;
}
