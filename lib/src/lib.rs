//! Simple traits for builtin enum items.
//! Primarily used by `enum_traits_macros` when automatically deriving types.
//! The crate `enum_traits_macros` is required for the derives.

#![cfg_attr(feature = "no_std" ,no_std)]
#![cfg_attr(feature = "nightly",feature(associated_consts))]

#[cfg(not(feature = "no_std"))]use  std::{borrow};
#[cfg(feature = "no_std")     ]use core::{borrow};

/// Represents the type used for indexing the variants of the enum item.
///`Type` should be an primitive integer type and have more values or an equal number of values compared to the number of variants in the enum item.
///
/// Derive this trait for an enum automatically using `#[derive(EnumIndex)]`
/// When derived, `Type` becomes the type specified in the `repr` attribute for the enum item.
/// If a `repr` attribute does not exist, a calculated minimum integer type based on the number of variant fields is used instead.
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
#[cfg(feature = "nightly")]
pub trait Len{
	/// Number of variants in an enum
	const LEN: usize;

	#[inline(always)]
	fn len() -> usize{<Self as Len>::LEN}
}
#[cfg(not(feature = "nightly"))]
pub trait Len{
	/// Number of variants in an enum
	fn len() -> usize;
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

/// Derive this trait for an enum automatically using `#[derive(EnumDiscriminant)]`
/// When this trait is derived, non-unit variants will be mapped to `None` in `from_discriminant`, and non-explicitly-specified discriminants will also be mapped to `None`.
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumDiscriminant)]
/// enum Enum{
/// 	A = 1,
/// 	B = 2,
/// 	C = 4,
/// 	D = 8,
/// 	E = 16,
/// 	F = 33,
/// 	G,
/// }
/// ```
///
/// # Example with manual impl
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{
/// 	A = 1,
/// 	B = 2,
/// 	C = 4,
/// 	D = 8,
/// 	E = 16,
/// 	F = 33,
/// 	G,
/// }
///
/// impl Discriminant for Enum{
/// 	type Type = usize;
///
/// 	fn from_discriminant(discriminant: <Self as Discriminant>::Type) -> Option<Self>{
/// 		Some(match discriminant {
/// 			 1  => Enum::A,
/// 			 2  => Enum::B,
/// 			 4  => Enum::C,
/// 			 8  => Enum::D,
/// 			 16 => Enum::E,
/// 			 33 => Enum::F,
/// 			 _ => return None,
/// 		 })
/// 	}
///
/// 	unsafe fn from_discriminant_unchecked(discriminant: <Self as Discriminant>::Type) -> Self{
/// 		match discriminant{
/// 			1  => Enum::A,
/// 			2  => Enum::B,
/// 			4  => Enum::C,
/// 			8  => Enum::D,
/// 			16 => Enum::E,
/// 			33 => Enum::F,
/// 			_ => ::std::mem::uninitialized(),
/// 		}
/// 	}
/// }
/// ```
pub trait Discriminant: Sized{
	/// The type of the discriminant
	type Type;

	/// Tries to construct an enum from the discriminant of the variants/enum items
	fn from_discriminant(discriminant: <Self as Discriminant>::Type) -> Option<Self>;

	/// Constructs an enum from the discriminant of the variants/enum items
	unsafe fn from_discriminant_unchecked(discriminant: <Self as Discriminant>::Type) -> Self;
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

/// Derive this trait for an enum automatically using `#[derive(EnumBitPattern)]`
///
/// # Example with derive
///
/// ```rust,ignore
/// #[macro_use]extern crate enum_traits_macros;
///
/// #[derive(EnumBitPattern)]
/// enum Enum{A,B(u8),C{c: u16},D,E(u32),F{f: u64},G,H(i8),I{i: i16}}
/// ```
///
/// # Example with manual impl
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B(u8),C{c: u16},D,E(u32),F{f: u64},G,H(i8),I{i: i16}}
///
/// impl BitPattern for Enum{
/// 	type ByteArray = [u8; 2];
///
/// 	fn bit_pattern(self) -> Self::ByteArray{
/// 		match self{
/// 			Enum::A     => [0b00000000 , 0b00000001],
/// 			Enum::B(..) => [0b00000000 , 0b00000010],
/// 			Enum::C{..} => [0b00000000 , 0b00000100],
/// 			Enum::D     => [0b00000000 , 0b00001000],
/// 			Enum::E(..) => [0b00000000 , 0b00010000],
/// 			Enum::F{..} => [0b00000000 , 0b00100000],
/// 			Enum::G     => [0b00000000 , 0b01000000],
/// 			Enum::H(..) => [0b00000000 , 0b10000000],
/// 			Enum::I{..} => [0b00000001 , 0b00000000],
/// 		}
/// 	}
/// 	fn bit_pattern_rev(self) -> Self::ByteArray{
/// 		match self{
/// 			Enum::A     => [0b10000000 , 0b00000000],
/// 			Enum::B(..) => [0b01000000 , 0b00000000],
/// 			Enum::C{..} => [0b00100000 , 0b00000000],
/// 			Enum::D     => [0b00010000 , 0b00000000],
/// 			Enum::E(..) => [0b00001000 , 0b00000000],
/// 			Enum::F{..} => [0b00000100 , 0b00000000],
/// 			Enum::G     => [0b00000010 , 0b00000000],
/// 			Enum::H(..) => [0b00000001 , 0b00000000],
/// 			Enum::I{..} => [0b00000000 , 0b10000000],
/// 		}
/// 	}
/// }
/// ```
pub trait BitPattern{
	type ByteArray;//TODO: : ::core::array::FixedSizeArray<u8> or : borrow::Borrow<[u8]>+borrow::BorrowMut<[u8]>+conv::AsRef<[u8]>+conv::AsMut<[u8]>;

	/// Bit pattern of the currently instantiated variant in the defined order of an enum
	/// Most significant bit first (e.g. 1000 is 8, 0100 is 4, 0010 is 2, 0001 is 1)
	/// The byte order of the array follows the bit order
	fn bit_pattern(self) -> Self::ByteArray;

	/// Bit pattern of the currently instantiated variant in the defined order of an enum
	/// Least significant bit first (e.g. 1000 is 1, 0100 is 2, 0010 is 4, 0001 is 8)
	/// The byte order of the array follows the bit order
	fn bit_pattern_rev(self) -> Self::ByteArray;
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
