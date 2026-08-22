//! Derives additional functionalities for enum items using procedural macros.
//!
//! Also see [`enum_traits`] for some of the traits that this library derives.
//!
//! There are also macros that work independently of [`enum_traits`]. See the documentation for each of the macros for more information.
//!
//! All of the functionality in this library can be toggled by using feature flags. See `Cargo.toml` for all the flags. This can be useful if only a few macros are used or if some features break in the future.

//#![cfg_attr(test,feature(non_exhaustive_omitted_patterns_lint))]

#![allow(non_snake_case)]
#![no_std]

extern crate alloc;
#[macro_use] extern crate quote;

#[cfg(any(feature = "derive_ends"             ,feature = "attr_ends"))]              mod enum_ends;
#[cfg(any(feature = "derive_field_structs"    ,feature = "attr_field_structs"))]     mod enum_field_structs;
#[cfg(any(feature = "derive_from_discriminant",feature = "attr_from_discriminant"))] mod enum_from_discriminant;
#[cfg(any(feature = "derive_from_index"       ,feature = "attr_from_index"))]        mod enum_from_index;
#[cfg(any(feature = "derive_from"             ,feature = "attr_from"))]              mod enum_from;
#[cfg(any(feature = "derive_from_str"         ,feature = "attr_from_str"))]          mod enum_from_str;
#[cfg(any(feature = "derive_index"            ,feature = "attr_index"))]             mod enum_index;
#[cfg(any(feature = "derive_is"               ,feature = "attr_is"))]                mod enum_is;
#[cfg(any(feature = "derive_iterable"         ,feature = "attr_iterable"))]          mod enum_iterable;
#[cfg(any(feature = "derive_iterator"         ,feature = "attr_iterator"))]          mod enum_iterator;
#[cfg(any(feature = "derive_len"              ,feature = "attr_len"))]               mod enum_len;
#[cfg(any(feature = "derive_tag"              ,feature = "attr_tag"))]               mod enum_tag;
#[cfg(any(feature = "derive_to_index"         ,feature = "attr_to_index"))]          mod enum_to_index;
#[cfg(any(feature = "derive_variant_name"     ,feature = "attr_variant_name"))]      mod enum_variant_name;
#[cfg(any(feature = "derive_step"             ,feature = "attr_step"))]              mod enum_step;
#[cfg(any(feature = "derive_variants_array"   ,feature = "attr_variants_array"))]    mod enum_variants_array;
mod util;

use proc_macro2::TokenStream;
use syn::ItemEnum;

#[inline(always)]
fn derive_enum<F>(input: proc_macro::TokenStream,gen_derive: F) -> proc_macro::TokenStream
	where F: FnOnce(ItemEnum) -> TokenStream
{
	let item = syn::parse2::<ItemEnum>(input.into()).expect("`derive(Enum*)` may only be applied to enum items");
	proc_macro::TokenStream::from(gen_derive(item))
}

#[inline(always)]
fn attr_enum<F>(attr: proc_macro::TokenStream,input: proc_macro::TokenStream,gen_derive: F) -> proc_macro::TokenStream
	where F: FnOnce(TokenStream,ItemEnum) -> TokenStream
{
	let item = syn::parse::<ItemEnum>(input.into()).expect("`impl_enum_*` may only be applied to enum items");
	proc_macro::TokenStream::from(gen_derive(attr.into(),item))
}

/// Implements [`enum_traits::Len`](../enum_traits/trait.Len.html).
///
/// The length is computed from the number of variants in the enum item.
///
/// # Requirements
/// - The derived item is an enum.
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
#[cfg(feature = "derive_len")]
#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_len::gen_derive)}

/// Implements a length constant to the enum.
///
/// The length is computed from the number of variants in the enum item.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Syntax
/// `#[impl_enum_len(<OuterAttribute*> <Visibility?> <Identifier>)]`
///
/// # Example using attributes
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_len(LENGTH)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const LENGTH: usize = 6;
/// }
/// ```
#[cfg(feature = "attr_len")]
#[proc_macro_attribute]
pub fn impl_enum_len(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_len::gen_attr)
}

/// Implements [`enum_traits::Ends`](../enum_traits/trait.Ends.html).
///
/// The ends are computed by using the first and the last variant of the enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum has at least one variant.
/// - The enum's first variant is a unit variant.
/// - The enum's last variant is a unit variant.
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
/// 	const FIRST: Self = Enum::A;
/// 	const LAST: Self = Enum::F;
/// }
/// ```
#[cfg(feature = "derive_ends")]
#[proc_macro_derive(EnumEnds)]
pub fn derive_EnumEnds(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_ends::gen_derive)}

/// Implements a constant to the first variant in the enum.
///
/// The ends are computed by using the first variant of the enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum has at least one variant.
/// - The enum's first variant is a unit variant.
///
/// # Syntax
/// `#[impl_enum_first(<OuterAttribute*> <Visibility?> <Identifier>)]`
///
/// # Example using attributes
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_first(START)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const START: Self = Enum::A;
/// }
/// ```
#[cfg(feature = "attr_ends")]
#[proc_macro_attribute]
pub fn impl_enum_first(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_ends::gen_attr_first)
}

/// Implements a constant to the last variant in the enum.
///
/// The ends are computed by using the last variant of the enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum has at least one variant.
/// - The enum's last variant is a unit variant.
///
/// # Syntax
/// `#[impl_enum_last(<OuterAttribute*> <Visibility?> <Identifier>)]`
///
/// # Example using attributes
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_last(END)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const END: Self = Enum::F;
/// }
/// ```
#[cfg(feature = "derive_ends")]
#[proc_macro_attribute]
pub fn impl_enum_last(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_ends::gen_attr_last)
}

/// Implements [`enum_traits::ToIndex`](../enum_traits/trait.ToIndex.html).
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum.
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
#[cfg(feature = "derive_to_index")]
#[proc_macro_derive(EnumToIndex)]
pub fn derive_EnumToIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_to_index::gen_derive)}

#[cfg(feature = "attr_to_index")]
#[proc_macro_attribute]
pub fn impl_enum_to_index(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_to_index::gen_attr)
}

/// Implements [`enum_traits::FromIndex`](../enum_traits/trait.FromIndex.html).
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum.
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
#[cfg(feature = "derive_from_index")]
#[proc_macro_derive(EnumFromIndex)]
pub fn derive_EnumFromIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_from_index::gen_derive)}

/// Implements [`enum_traits::Index`](../enum_traits/trait.Index.html).
///
/// [`enum_traits::Index::Type`](../enum_traits/trait.Index.html#associatedtype.Type) is computed by the smallest unsigned integer type fitting the number of variants minus one.
///
/// # Requirements
///
/// - The derived item is an enum.
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIndex)]
/// enum Enum{A,B,C,D,E,F}
///
/// #[derive(EnumIndex)]
/// enum Enum2{A = 1000,B = 50000,C,D,E,F}
///
/// #[derive(EnumIndex)]
/// #[repr(u64)]
/// enum Enum3{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// enum Enum2{A = 1000,B = 50000,C,D,E,F}
///
/// #[repr(u64)]
/// enum Enum3{A,B,C,D,E,F}
///
/// impl Index for Enum{
/// 	type Type = u8;
/// }
///
/// impl Index for Enum2{
/// 	type Type = u8;
/// }
///
/// impl Index for Enum3{
/// 	type Type = u8;
/// }
/// ```
#[cfg(feature = "derive_index")]
#[proc_macro_derive(EnumIndex)]
pub fn derive_EnumIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	derive_enum(input,enum_index::gen_derive)
}

/// Creates a struct representing the iterator state and implements [`enum_traits::Iterable`](../enum_traits/trait.Iterable.html).
///
/// A struct named `<name of Self> + "Iter"` will be generated with the same visibility as `Self`.
/// This struct will then implement [`core::iter::Iterator`] and [`core::iter::Iterator::Item`] will be assigned to it when implementing [`enum_traits::Iterable`](../enum_traits/trait.Iterable.html) for `Self`.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum variants are all unit variants.
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIterable)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
/// enum Enum{A,B,C,D,E,F}
///
/// struct EnumIterable(pub Option<Enum>);
///
/// impl Iterable for Enum{
/// 	type Iter = EnumIterable;
///
/// 	fn variants() -> Self::Iter { EnumIterable(None) }
/// }
///
/// impl Iterator for EnumIterable{
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
#[cfg(feature = "derive_iterable")]
#[proc_macro_derive(EnumIterable)]
pub fn derive_EnumIter(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_iterable::gen_derive)}

/// Implements [`core::iter::Iterator`].
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum variants are all unit variants.
#[cfg(feature = "derive_iterator")]
#[proc_macro_derive(EnumIterator)]
pub fn derive_EnumIterator(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_iterator::gen_derive)}

/// Implements [`enum_traits::EnumVariantName`](../enum_traits/trait.VariantName.html).
///
/// The names are generated from the variant names.
///
/// # Requirements
/// - The derived item is an enum.
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
#[cfg(feature = "derive_variant_name")]
#[proc_macro_derive(EnumVariantName)]
pub fn derive_EnumVariantName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {derive_enum(input,enum_variant_name::gen_derive)}

#[cfg(feature = "attr_variant_name")]
#[proc_macro_attribute]
pub fn impl_enum_variant_name(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_variant_name::gen_attr)
}

/// Implements [`core::str::FromStr`].
///
/// Note that only unit variants will be constructable from strings using `from_str`.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
///	```rust
/// use core::str::FromStr;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromStr,Debug,PartialEq)]
/// enum Enum {
/// 	A,
/// 	B(i32),
/// 	C{speed: f32},
/// 	D,
/// 	E,
/// }
///
/// assert_eq!(Ok(Enum::A) , Enum::from_str("A"));
/// assert_eq!(Err(())     , Enum::from_str("B"));
/// assert_eq!(Err(())     , Enum::from_str("C"));
/// assert_eq!(Ok(Enum::D) , Enum::from_str("D"));
/// assert_eq!(Ok(Enum::E) , Enum::from_str("E"));
/// assert_eq!(Err(())     , Enum::from_str("F"));
/// ```
///
/// # Expanded example
///
///	```rust
/// use core::str::FromStr;
///
/// enum Enum {
/// 	A,
/// 	B(i32),
/// 	C{speed: f32},
/// 	D,
/// 	E,
/// }
///
/// impl FromStr for Enum{
///		type Err = ();
///
///		fn from_str(str: &str) -> Result<Self,Self::Err>{
///			Ok(match str{
///				"A" => Enum::A,
///				"D" => Enum::D,
///				"E" => Enum::E,
///				_ => return Err(())
///			})
///		}
///	}
///
/// // <rest is omitted>
///	```
#[cfg(feature = "derive_from_str")]
#[proc_macro_derive(EnumFromStr)]
pub fn derive_EnumFromStr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {derive_enum(input,enum_from_str::gen_derive)}

/// Creates an enum with unit variants from the derived enum, and implements [`enum_traits::Tag`](../enum_traits/trait.Tag.html).
///
/// An enum named `<name of Self> + "Tag"` will be created with the same visibility as `Self`.
/// This enum will then be assigned to [`enum_traits::Tag::Tag`](../enum_traits/trait.Tag.html#associatedtype.Tag) when implementing [`enum_traits::Tag`](../enum_traits/trait.Tag.html) for `Self`.
///
/// # Arguments
/// A supplemental attribute `enum_tag` specifies options in the generated code using the following syntax:
///
/// `#[enum_tag(<OuterAttribute*> <Visibility?> enum <Identifier>)]`
///
/// The arguments are as follows:
/// - Attributes on the tag enum (default is `#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]`).
/// - The visibility of the tag enum (default is the same as the original enum).
/// - The name of the tag enum (default is `<name of Self> + "Tag"`).
///
/// # Requirements
/// - The derived item is an enum.
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
///
/// #[derive(EnumTag)]
/// #[enum_tag(#[deprecated] pub(crate) enum Enum2HasACustomName)]
/// enum Enum2{
/// 	A,
/// 	B
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
///
/// enum Enum2{
/// 	A,
/// 	B
/// }
///
/// #[deprecated] pub(crate) enum Enum2HasACustomName{A,B}
///
/// impl Tag for Enum2{
/// 	type Tag = Enum2HasACustomName;
///
/// 	fn into_tag(self) -> Self::Tag{
/// 		match self {
/// 			Enum2::A => Enum2HasACustomName::A,
/// 			Enum2::B => Enum2HasACustomName::B,
/// 		}
/// 	}
///
/// 	fn tag(&self) -> Self::Tag{
/// 		match self {
/// 			&Enum2::A => Enum2HasACustomName::A,
/// 			&Enum2::B => Enum2HasACustomName::B,
/// 		}
/// 	}
/// }
/// ```
#[cfg(feature = "derive_tag")]
#[proc_macro_derive(EnumTag,attributes(enum_tag))]
pub fn derive_EnumTag(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_tag::gen_derive)}

/// Implements functions that checks if an value of the enum matches a certain variant.
///
/// The generated names of the functions are converted from CamelCase to snake_case using [util::camelcase_to_snakecase].
///
/// # Arguments
/// A supplemental attribute `enum_is` specifies options in the generated code using the following syntax:
///
/// `#[enum_is(<OuterAttribute*> <Visibility?> fn <Identifier>)]`
///
/// The arguments are as follows:
/// - Attributes on every function.
/// - The visibility of every function (default is the same as the enum).
/// - The additional prefix to every function name (default is `is_`).
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIs)]
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	RobotInDisguise{speed: f32},
/// }
///
/// assert!(Enum::Dog.is_dog());
/// assert!(Enum::Cat(0).is_cat());
/// assert!(Enum::RobotInDisguise{speed: 0.0}.is_robot_in_disguise());
///
/// assert!(!Enum::Dog.is_cat());
/// assert!(!Enum::Dog.is_robot_in_disguise());
/// assert!(!Enum::RobotInDisguise{speed: 0.0}.is_cat());
/// assert!(!Enum::RobotInDisguise{speed: 0.0}.is_dog());
/// assert!(!Enum::Cat(0).is_dog());
/// assert!(!Enum::Cat(0).is_robot_in_disguise());
///
/// #[derive(EnumIs)]
/// #[enum_is(#[deprecated] pub(crate) fn prefix_)]
/// enum Enum2{
/// 	A,
/// 	B
/// }
///
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	RobotInDisguise{speed: f32},
/// }
///
/// impl Enum{
/// 	fn is_dog(&self) -> bool{if let &Enum::Dog = self {true} else {false}}
/// 	fn is_cat(&self) -> bool{if let &Enum::Cat(_) = self {true} else {false}}
/// 	fn is_robot_in_disguise(&self) -> bool{if let &Enum::RobotInDisguise{..} = self {true} else {false}}
/// }
///
/// // <rest is omitted>
///
/// enum Enum2{
/// 	A,
/// 	B
/// }
///
/// impl Enum2 {
///     #[deprecated] pub(crate) fn prefix_a(&self) -> bool{
///         if let &Enum2::A = self {true} else {false}
///     }
///     #[deprecated] pub(crate) fn prefix_b(&self) -> bool{
///         if let &Enum2::B = self {true} else {false}
///     }
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_is")]
#[proc_macro_derive(EnumIs,attributes(enum_is))]
pub fn derive_EnumIs(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_is::gen_derive)}

/// Implements [`core::convert::From`] for all variants in the enum by using the types of the fields.
///
/// If more than one field exist in a variant, a tuple is used to represent the variant.
///
/// Both tuple fields and record fields become tuples in the `impl From`:
///
/// - `enum Enum{A{x: i8}}`
/// - `enum Enum{A(i8)}`
///
/// both generate `impl From<i8>`.
///
/// A unit variant (with no fields) is represented by the unit type (an empty tuple) (`enum Enum{A}` generates `impl From<()>`).
///
/// # Requirements
/// - The derived item is an enum.
/// - There are no variants with the same types in their fields (after type parameters are applied and converted to tuples). Otherwise, "conflicting implementations of trait From<_>" errors will be reported when compiling the generated code.
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumFrom,Debug,PartialEq,Eq)]
/// enum Enum{
///     A,
///     B(u8),
///     C(u8,u16,u32,u64),
///     D{d: i8},
///     E{a: i8 , b: i16 , c: i32},
///     F{a: i8 , b: i32 , c: i16},
/// }
///
/// assert_eq!(Enum::A                     , Enum::from(()));
/// assert_eq!(Enum::B(6)                  , Enum::from(6u8));
/// assert_eq!(Enum::C(0,1,2,3)            , Enum::from((0u8,1u16,2u32,3u64)));
/// assert_eq!(Enum::D{d: 3}               , Enum::from(3i8));
/// assert_eq!(Enum::E{a: 3 , b: 5 , c: 7} , Enum::from((3i8,5i16,7i32)));
/// assert_eq!(Enum::F{a: 9 , b: 7 , c: 6} , Enum::from((9i8,7i32,6i16)));
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum{
///     A,
///     B(u8),
///     C(u8,u16,u32,u64),
///     D{d: i8},
///     E{a: i8 , b: i16 , c: i32},
///     F{a: i8 , b: i32 , c: i16},
/// }
///
/// impl From<()> for Enum{
///     #[inline(always)] fn from((): ()) -> Self{
///         Enum::A
///     }
/// }
///
/// impl From<u8> for Enum{
///     #[inline(always)] fn from(x0: u8) -> Self{
///         Enum::B(x0)
///     }
/// }
///
/// impl From<(u8,u16,u32,u64)> for Enum{
///     #[inline(always)] fn from((x0,x1,x2,x3): (u8,u16,u32,u64)) -> Self{
///         Enum::C(x0,x1,x2,x3)
///     }
/// }
///
/// impl From<i8> for Enum{
///     #[inline(always)] fn from(x0: i8) -> Self{
///         Enum::D{d: x0}
///     }
/// }
///
/// impl From<(i8,i16,i32)> for Enum{
///     #[inline(always)] fn from((x0,x1,x2): (i8,i16,i32)) -> Self{
///         Enum::E{a: x0 , b: x1 , c: x2}
///     }
/// }
///
/// impl From<(i8,i32,i16)> for Enum{
///     #[inline(always)] fn from((x0,x1,x2): (i8,i32,i16)) -> Self{
///         Enum::F{a: x0 , b: x1 , c: x2}
///     }
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_from")]
#[proc_macro_derive(EnumFrom)]
pub fn derive_EnumFrom(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_from::gen_derive)}

/// Implements [`enum_traits::Step`](../enum_traits/trait.Step.html).
///
/// The next and previous functions are computed by using the variants of the enum in their defined order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum's variants are all unit variants.
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumStep)]
/// enum Enum{A,B,C,D}
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D}
///
/// impl Step for Enum{
/// 	fn next(self) -> Option<Self>{match self{
/// 		Enum::A => Some(Enum::B),
/// 		Enum::B => Some(Enum::C),
/// 		Enum::C => Some(Enum::D),
/// 		Enum::D => None,
/// 	}}
/// 	fn previous(self) -> Option<Self>{match self{
/// 		Enum::A => None,
/// 		Enum::B => Some(Enum::A),
/// 		Enum::C => Some(Enum::B),
/// 		Enum::D => Some(Enum::C),
/// 	}}
/// }
/// ```
#[cfg(feature = "derive_step")]
#[proc_macro_derive(EnumStep)]
pub fn derive_EnumStep(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_step::gen_derive)}

/// Implements [`enum_traits::FromDiscriminant`](../enum_traits/trait.FromDiscriminant.html) for any discriminant type.
///
/// The implementation is using pattern matching on the values of [`enum_traits::IntoDiscriminant`](../enum_traits/trait.IntoDiscriminant.html) and therefore requires an explicit `impl IntoDiscriminant` of the desired discriminant type.
///
/// See [`enum_traits::impl_IntoDiscriminant_of_numeric`](../enum_traits/macro.impl_IntoDiscriminant_of_numeric.html) for a standard definition of [`IntoDiscriminant`](../enum_traits/trait.IntoDiscriminant.html).
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum variants are all unit variants.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromDiscriminant,Debug,PartialEq,Eq)]
/// #[repr(u8)]
/// pub enum Enum{
/// 	A = 8,
/// 	B,
/// 	C = 5,
/// 	D = 7,
/// }
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[repr(u8)]
/// pub enum Enum{
/// 	A = 8,
/// 	B,
/// 	C = 5,
/// 	D = 7,
/// }
///
/// impl<T> FromDiscriminant<T> for Enum where
/// 	Self: IntoDiscriminant<T>,
/// 	T: PartialEq
/// {
/// 	#[inline]
/// 	fn from_discriminant(discriminant: T) -> Option<Self>{
/// 		match discriminant{
/// 			n if n == Enum::A.into_discriminant() => Some(Enum::A),
/// 			n if n == Enum::B.into_discriminant() => Some(Enum::B),
/// 			n if n == Enum::C.into_discriminant() => Some(Enum::C),
/// 			n if n == Enum::D.into_discriminant() => Some(Enum::D),
/// 			_ => None,
/// 		}
/// 	}
///
/// 	#[inline]
/// 	unsafe fn from_discriminant_unchecked(discriminant: T) -> Self{
/// 		match discriminant{
/// 			n if n == Enum::A.into_discriminant() => Enum::A,
/// 			n if n == Enum::B.into_discriminant() => Enum::B,
/// 			n if n == Enum::C.into_discriminant() => Enum::C,
/// 			n if n == Enum::D.into_discriminant() => Enum::D,
/// 			_ => ::core::hint::unreachable_unchecked(),
/// 		}
/// 	}
/// }
///
/// // <rest is omitted>
/// ```
/// # LLVM IR example
///
/// This shows how `from_discriminant_unchecked` becomes an identity.
///
/// Compiler version: rustc 1.66.0-nightly (57f097ea2 2022-10-01)
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromDiscriminant,Debug,PartialEq,Eq)]
/// #[repr(u8)]
/// pub enum Enum{
/// 	A = 8,
/// 	B,
/// 	C = 5,
/// 	D = 7,
/// }
///
/// impl IntoDiscriminant<u8> for Enum{
/// 	#[inline(always)] fn into_discriminant(self) -> u8{self as u8}
/// }
///
/// #[unsafe(no_mangle)]
/// #[inline(never)]
/// pub fn llvm_ir_example1(n: u8) -> Option<Enum>{
/// 	Enum::from_discriminant(n)
/// }
///
/// #[unsafe(no_mangle)]
/// #[inline(never)]
/// pub unsafe fn llvm_ir_example2(n: u8) -> Enum{
/// 	Enum::from_discriminant_unchecked(n)
/// }
///
/// #[unsafe(no_mangle)]
/// #[inline(never)]
/// pub fn llvm_ir_example3(n: ::core::mem::Discriminant<Enum>) -> Option<Enum>{
/// 	Enum::from_discriminant(n)
/// }
///
/// #[unsafe(no_mangle)]
/// #[inline(never)]
/// pub unsafe fn llvm_ir_example4(n: ::core::mem::Discriminant<Enum>) -> Enum{
/// 	Enum::from_discriminant_unchecked(n)
/// }
/// ```
///
/// ```llvm
/// ; Function Attrs: mustprogress nofree noinline norecurse nosync nounwind nonlazybind readnone willreturn uwtable
/// define noundef i8 @llvm_ir_example1(i8 %n) unnamed_addr #0 personality ptr @rust_eh_personality {
/// start:
/// 	%switch.tableidx = add i8 %n, -5
/// 	%0 = icmp ult i8 %switch.tableidx, 5
/// 	%switch.cast = zext i8 %switch.tableidx to i40
/// 	%switch.shiftamt = shl nuw nsw i40 %switch.cast, 3
/// 	%switch.downshift = lshr i40 38789383173, %switch.shiftamt
/// 	%switch.masked = trunc i40 %switch.downshift to i8
/// 	%.0.i = select i1 %0, i8 %switch.masked, i8 4
/// 	ret i8 %.0.i
/// }
///
/// ; Function Attrs: mustprogress nofree noinline norecurse nosync nounwind nonlazybind readnone willreturn uwtable
/// define noundef i8 @llvm_ir_example2(i8 returned %n) unnamed_addr #0 personality ptr @rust_eh_personality {
/// start:
/// 	ret i8 %n
/// }
///
/// @llvm_ir_example3 = unnamed_addr alias i8 (i8), ptr @llvm_ir_example1
/// @llvm_ir_example4 = unnamed_addr alias i8 (i8), ptr @llvm_ir_example2
///
/// ; <rest is omitted>
/// ```
#[cfg(feature = "derive_from_discriminant")]
#[proc_macro_derive(EnumFromDiscriminant)]
pub fn derive_EnumFromDiscriminant(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	derive_enum(input,|item| {
		//let ty = util::minimum_type_containing_enum(&item);
		enum_from_discriminant::gen_derive(item)
	})
}

/// Creates structs for each variant's fields and implements [`core::convert::From`] for every struct to the enum.
///
/// Structs named `<name of variant>` will be created with the same visibility as `Self`.
///
/// # Arguments
/// A supplemental attribute `enum_field_structs` specifies options in the generated code using the following syntax:
///
/// `#[enum_field_structs(<OuterAttribute*> <Visibility?> struct <Identifier> <Identifier>)]`
///
/// The arguments are as follows:
/// - Attributes on every struct.
/// - The visibility of every struct (default is the same as the original enum).
/// - The additional prefix to every struct.
/// - The additional postfix to every struct.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
/// //TODO
/// #[derive(EnumFieldStructs)]
/// enum Fields<'a,X,Y>{
/// 	A(i8),
/// 	B(i32),
/// 	C(u8,u16,u32),
/// 	D{d: (u8,i32)},
/// 	E{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
/// 	F,
/// 	G(X),
/// 	H(&'a Y),
/// 	I{x: X,y: &'a Y},
/// 	J{x: i8},
/// }
///
/// #[derive(EnumFieldStructs)]
/// #[enum_field_structs(#[deprecated] pub(crate) enum Enum2HasACustomName)]
/// enum Enum2{
/// 	X,
/// 	Y
/// }
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// ```
#[cfg(feature = "derive_field_structs")]
#[proc_macro_derive(EnumFieldStructs)]
pub fn derive_EnumFieldStructs(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_field_structs::gen_derive)}

/// Implements [`enum_traits::VariantsArray`](../enum_traits/trait.VariantsArray.html).
///
/// The array is constructed by the variants in order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum's variants are all unit variants.
///
/// # Example using derive
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumVariantsArray,EnumLen)]
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
///
/// impl VariantsArray for Enum{
/// 	const variants: [Self; <Self as Len>::LEN] = [Self::A,Self::B,Self::C,Self::D,Self::E,Self::F];
/// }
/// ```
#[cfg(feature = "derive_variants_array")]
#[proc_macro_derive(EnumVariantsArray)]
pub fn derive_EnumVariantsArray(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_variants_array::gen_derive)}

/// Implements an array constant that lists every variant of the enum in order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_variants_array(<OuterAttribute*> <Visibility?> <Identifier>)]`
///
/// # Example using attributes
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_variants_array(VARIANTS)]
/// enum Enum{A,B,C,D,E,F}
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const VARIANTS: [Self; 6] = [Enum::A,Enum::B,Enum::C,Enum::D,Enum::E,Enum::F];
/// }
/// ```
#[cfg(feature = "attr_variants_array")]
#[proc_macro_attribute]
pub fn impl_enum_variants_array(attr: proc_macro::TokenStream,item: proc_macro::TokenStream) -> proc_macro::TokenStream{
	attr_enum(attr,item,enum_variants_array::gen_attr)
}
