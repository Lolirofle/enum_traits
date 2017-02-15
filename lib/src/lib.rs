//! Simple traits for builtin enum items.
//! Primarily used by `enum_traits_macros` when automatically deriving types.
//! The crate `enum_traits_macros` is required for the derives.

#![cfg_attr(not(feature = "stable"),feature(associated_consts))]

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

///Constructors for an enum type from indices based on the variants' defined order
///
///Derive this trait for an enum automatically using `#[derive(EnumFromIndex)]`
pub trait FromIndex: Index + Sized{
	///Tries to construct `Self` from an index based on the variants' defined order
	fn from_index(index: <Self as Index>::Type) -> Option<Self>;

	///Constructs `Self` from an index based on the variants' defined order
	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self;
}

///Indices for an enum type based on the variants' defined order
///
///Derive this trait for an enum automatically using `#[derive(EnumToIndex)]`
pub trait ToIndex: Index{
	///Index in the defined order of an enum
	fn into_index(self) -> <Self as Index>::Type;

	///Index in the defined order of an enum
	fn index(&self) -> <Self as Index>::Type;
}

///Number of variants in an enum type
///
///Derive this trait for an enum automatically using `#[derive(EnumLen)]`
#[cfg(not(feature = "stable"))]
pub trait Len{
	///Number of variants in an enum
	const LEN: usize;

	#[inline(always)]
	fn len() -> usize{<Self as Len>::LEN}
}
#[cfg(feature = "stable")]
pub trait Len{
	///Number of variants in an enum
	fn len() -> usize;
}

///Constructors for an enum type from its endpoints based on the variants' defined order
///
///Derive this trait for an enum automatically using `#[derive(EnumEnds)]`
pub trait Ends: Sized{
	///The first variant in the defined order of an enum
	fn first() -> Self;

	///The last variant in the defined order of an enum
	fn last() -> Self;
}

///Derive this trait for an enum automatically using `#[derive(EnumDiscriminant)]`
///When this trait is derived, non-unit variants will be mapped to `None` in `from_discriminant`.
pub trait Discriminant: Sized{
	///The type of the discriminant
	type Type;

	///Tries to construct an enum from the discriminant of the variants/enum items
	fn from_discriminant(discriminant: <Self as Discriminant>::Type) -> Option<Self>;

	///Constructs an enum from the discriminant of the variants/enum items
	unsafe fn from_discriminant_unchecked(discriminant: <Self as Discriminant>::Type) -> Self;
}

///Derive this trait for an enum automatically using `#[derive(EnumIter)]`
///When derived, a struct named ((name of Self) + "Iter") will be created with the same visibility as `Self`.
///This struct will then implement `Iterator` and `Iter` will be assigned to it when implementing `Iterable` for `Self`.
pub trait Iterable{
	///The type of the iterator
	type Iter: Iterator;

	///Constructs an iterator that iterates over every variant in the defined order
	fn variants() -> Self::Iter;
}

///Derive this trait for an enum automatically using `#[derive(EnumVariantName)]`
pub trait VariantName{
	///The name of the currently instantiated variant
	fn variant_name(&self) -> &'static str;
}

///Derive this trait for an enum automatically using `#[derive(EnumBitPattern)]`
pub trait BitPattern{
	type ByteArray;

	///Bit pattern of the currently instantiated variant in the defined order of an enum
	///Most significant bit first (e.g. 1000 is 8)
	fn bit_pattern(self) -> Self::ByteArray;

	///Bit pattern of the currently instantiated variant in the defined order of an enum
	///Least significant bit first (e.g. 1000 is 1)
	fn bit_pattern_rev(self) -> Self::ByteArray;
}

///Derive this trait for an enum automatically using `#[derive(EnumKind)]`
pub trait Kind{
	type Kind;

	///Kind of the currently instantiated variant
	fn kind(&self) -> Self::Kind;
}
