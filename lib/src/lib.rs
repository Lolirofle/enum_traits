//! Traits for enum items.
//!
//! Used by the crate [`enum_traits_macros`] to automatically derive enums.
//!
//! # Import
//!
//! Add one of the following snippets to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! enum_traits = <VERSION>
//! enum_traits_macros = <VERSION>
//! ```
//! or
//! ```toml
//! [dependencies]
//! enum_traits = { version = <VERSION>, features = ["derive"] }
//! ```

#![no_std]

use core::borrow;

#[cfg(feature = "derive")]
pub use enum_traits_macros::*;

/// Represents the type used for indexing the variants of an enum item type.
///
/// This is primarily used by [`FromIndex`] and [`ToIndex`].
///
/// Derive this trait for an enum automatically using [`#[derive(EnumIndex)]`][EnumIndex].
///
/// # Requirements
///
/// - [`Type`][`Index::Type`] should be a primitive unsigned integer type.
/// - The number of variants of `Self` should be lesser than or equal the number of values of [`Type`][`Index::Type`].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIndex)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl enum_traits::Index for Enum{
/// 	type Type = u8;
/// }
/// ```
pub trait Index{
	/// Type used as an index for the variants of `Self`.
	type Type;
}

/// A constructor from an index based on an order on the variants of an enum type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumFromIndex)]`][EnumFromIndex].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromIndex,EnumIndex)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Index for Enum{
/// 	type Type = u8;
/// }
/// impl FromIndex for Enum {
/// 	fn from_index(index: <Self as Index>::Type) -> Option<Self> {
/// 		Some(match index{
/// 				 0 => Enum::A,
/// 				 1 => Enum::B,
/// 				 2 => Enum::C,
/// 				 3 => Enum::D,
/// 				 4 => Enum::E,
/// 				 5 => Enum::F,
/// 				 _ => return None,
/// 			 })
/// 	}
/// 	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self {
/// 		match index{
/// 			0 => Enum::A,
/// 			1 => Enum::B,
/// 			2 => Enum::C,
/// 			3 => Enum::D,
/// 			4 => Enum::E,
/// 			5 => Enum::F,
/// 			_ => unreachable!(),
/// 		}
/// 	}
/// }
/// ```
pub trait FromIndex: Index + Sized{
	/// Tries to construct `Self` from an index based on the variants' defined order.
	fn from_index(index: <Self as Index>::Type) -> Option<Self>;

	/// Constructs `Self` from an index based on the variants' defined order.
	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self;
}

/// Indices based on an order on the variants of an enum type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumToIndex)]`][EnumToIndex]
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIndex,EnumToIndex)]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// impl Index for Enum{
/// 	type Type = u8;
/// }
/// impl ToIndex for Enum{
/// 	fn into_index(self) -> <Self as Index>::Type{
/// 		match self{
/// 			Enum::A     => 0,
/// 			Enum::B(..) => 1,
/// 			Enum::C{..} => 2,
/// 			Enum::D     => 3,
/// 			Enum::E(..) => 4,
/// 			Enum::F{..} => 5,
/// 		}
/// 	}
/// 	fn index(&self) -> <Self as Index>::Type{
/// 		match self{
/// 			&Enum::A     => 0,
/// 			&Enum::B(..) => 1,
/// 			&Enum::C{..} => 2,
/// 			&Enum::D     => 3,
/// 			&Enum::E(..) => 4,
/// 			&Enum::F{..} => 5,
/// 		}
/// 	}
/// }
/// ```
pub trait ToIndex: Index{
	/// Index in the defined order of an enum
	fn into_index(self) -> <Self as Index>::Type;

	/// Index in the defined order of an enum
	fn index(&self) -> <Self as Index>::Type;
}

/// Number of variants in an enum type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumLen)]`][EnumLen].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumLen)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Len for Enum{
/// 	const LEN: usize = 6;
/// }
/// ```
pub trait Len{
	/// Number of variants in an item.
	const LEN: usize;
}

/// Constructors for the endpoints of an enum based on an order on the variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumEnds)]`][EnumEnds].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumEnds)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Ends for Enum{
/// 	fn first() -> Self { Enum::A }
/// 	fn last() -> Self { Enum::F }
/// }
/// ```
pub trait Ends: Sized{
	/// The first variant in the defined order of an enum.
	fn first() -> Self;

	/// The last variant in the defined order of an enum.
	fn last() -> Self;
}

/// An enum item type that have a corresponding iterator iterating over all variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumIter)]`][EnumIter].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIter)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
/// enum Enum{A,B,C,D,E,F}
///
/// struct EnumIter(pub Option<Enum>);
///
/// impl Iterable for Enum{
/// 	type Iter = EnumIter;
///
/// 	fn variants() -> Self::Iter { EnumIter(None) }
/// }
///
/// impl Iterator for EnumIter{
/// 	type Item = Enum;
///
/// 	fn next(&mut self) -> Option<Self::Item>{
/// 		Some(match &self.0{
/// 			&None => {
/// 				self.0 = Some(Enum::A);
/// 				Enum::A }
/// 			&Some(Enum::A) => {
/// 				self.0 = Some(Enum::B);
/// 				Enum::B
/// 			}
/// 			&Some(Enum::B) => {
/// 				self.0 = Some(Enum::C);
/// 				Enum::C
/// 			}
/// 			&Some(Enum::C) => {
/// 				self.0 = Some(Enum::D);
/// 				Enum::D
/// 			}
/// 			&Some(Enum::D) => {
/// 				self.0 = Some(Enum::E);
/// 				Enum::E
/// 			}
/// 			&Some(Enum::E) => {
/// 				self.0 = Some(Enum::F);
/// 				Enum::F
/// 			}
/// 			_ => return None,
/// 		})
/// 	}
/// }
/// ```
pub trait Iterable where
	<<Self as Iterable>::Iter as Iterator>::Item: borrow::Borrow<Self>
{
	/// The type of the iterator
	type Iter: Iterator;

	/// Constructs an iterator that iterates over every variant in the defined order
	fn variants() -> Self::Iter;
}

/// A function converting from a variant to its defined name.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumVariantName)]`][EnumVariantName].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumVariantName)]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// impl VariantName for Enum{
/// 	fn variant_name(&self) -> &'static str{
/// 		match self{
/// 			&Enum::A     => "A",
/// 			&Enum::B(..) => "B",
/// 			&Enum::C{..} => "C",
/// 			&Enum::D     => "D",
/// 			&Enum::E(..) => "E",
/// 			&Enum::F{..} => "F",
/// 		}
/// 	}
/// }
/// ```
pub trait VariantName{
	/// The name of the currently instantiated variant
	fn variant_name(&self) -> &'static str;
}

/// An enum item type that have a corresponding enum consisting of only unit variants describing the discriminants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumTag)]`][EnumTag].
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumTag)]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// enum EnumTag{A,B,C,D,E,F}
///
/// impl Tag for Enum{
/// 	type Tag = EnumTag;
///
/// 	fn into_tag(self) -> Self::Tag{
/// 		match self {
/// 			Enum::A     => EnumTag::A,
/// 			Enum::B(..) => EnumTag::B,
/// 			Enum::C{..} => EnumTag::C,
/// 			Enum::D     => EnumTag::D,
/// 			Enum::E(..) => EnumTag::E,
/// 			Enum::F{..} => EnumTag::F,
/// 		}
/// 	}
///
/// 	fn tag(&self) -> Self::Tag{
/// 		match self {
/// 			&Enum::A     => EnumTag::A,
/// 			&Enum::B(..) => EnumTag::B,
/// 			&Enum::C{..} => EnumTag::C,
/// 			&Enum::D     => EnumTag::D,
/// 			&Enum::E(..) => EnumTag::E,
/// 			&Enum::F{..} => EnumTag::F,
/// 		}
/// 	}
/// }
/// ```
pub trait Tag{
	type Tag;

	/// The tag (unit variant) of the currently instantiated variant
	fn tag(&self) -> Self::Tag;

	/// The tag (unit variant) of the currently instantiated variant
	fn into_tag(self) -> Self::Tag;
}
