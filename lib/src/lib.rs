//! Simple traits for builtin enum items.
//! Primarily used by `enum_traits_macros` when automatically deriving types.
//! The crate `enum_traits_macros` is required for the derives.

#![cfg_attr(feature = "no_std" ,no_std)]

#[cfg(not(feature = "no_std"))]use  std::borrow;
#[cfg(feature = "no_std")     ]use core::borrow;

/// Represents the type used for indexing the variants of the enum item.
///`Type` should be an primitive integer type and have more values or an equal number of values compared to the number of variants in the enum item.
///
/// Derive this trait for an enum automatically using `#[derive(EnumIndex)]`
/// When derived, `Type` becomes the type specified in the `repr` attribute for the enum item.
/// If a `repr` attribute does not exist, the smalest integer type based on the number of variant fields is used instead.
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumIndex)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Example with manual impl
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Index for Enum{
/// 	type Type = u8;
/// }
/// ```
pub trait Index{
	/// Type used as an index for the enum
	type Type;
}

/// Constructors for an enum type from indices based on the variants' defined order
///
/// Derive this trait for an enum automatically using `#[derive(EnumFromIndex)]`
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumFromIndex)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Example with manual impl
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
/// 			_ => ::std::mem::uninitialized(),
/// 		}
/// 	}
/// }
/// ```
pub trait FromIndex: Index + Sized{
	/// Tries to construct `Self` from an index based on the variants' defined order
	fn from_index(index: <Self as Index>::Type) -> Option<Self>;

	/// Constructs `Self` from an index based on the variants' defined order
	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self;
}

/// Indices for an enum type based on the variants' defined order
///
/// Derive this trait for an enum automatically using `#[derive(EnumToIndex)]`
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumToIndex)]
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
/// # Example with manual impl
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

/// Number of variants in an enum type
///
/// Derive this trait for an enum automatically using `#[derive(EnumLen)]`
///
/// # Example with derive
///
/// ```rust,ignorerust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumLen)]
/// enum Enum{A,B,C,D,E,F}
/// ```
pub trait Len{
	/// Number of variants in an enum
	const LEN: usize;
}

/// Constructors for an enum type from its endpoints based on the variants' defined order
///
/// Derive this trait for an enum automatically using `#[derive(EnumEnds)]`
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumEnds)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Example with manual impl
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
	/// The first variant in the defined order of an enum
	fn first() -> Self;

	/// The last variant in the defined order of an enum
	fn last() -> Self;
}

/// Derive this trait for an enum automatically using `#[derive(EnumIter)]`
/// When derived, a struct named ((name of Self) + "Iter") will be created with the same visibility as `Self`.
/// This struct will then implement `Iterator` and `Iter` will be assigned to it when implementing `Iterable` for `Self`.
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumIter)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Example with manual impl
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

/// Derive this trait for an enum automatically using `#[derive(EnumVariantName)]`
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
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
/// # Example with manual impl
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

/// Derive this trait for an enum automatically using `#[derive(EnumTag)]`
/// When derived, an enum named ((name of Self) + "Tag") will be created with the same visibility as `Self`.
/// This enum will then will be assigned to the `Iter` associated type when implementing `Tag` for `Self`.
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
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
/// # Example with manual impl
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
/// 	type Enum = EnumTag;
///
/// 	fn tag(&self) -> Self::Enum{
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
	type Enum;

	/// The tag (unit variant) of the currently instantiated variant
	fn tag(&self) -> Self::Enum;
}
