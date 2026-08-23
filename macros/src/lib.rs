//! Derives additional functionalities for enum items using procedural macros.
//!
//! Also see [`enum_traits`] for some of the traits that this library derives.
//!
//! This crate also include procedural macros that work independently of [`enum_traits`]. See the documentation for each of the attributes for more information.
//!
//! All of the functionality in this library can be toggled by using feature flags. See `Cargo.toml` for all the flags. This can be useful if only a few macros are used or if some features break in the future.

#![allow(non_snake_case)]
#![no_std]

extern crate alloc;
#[macro_use] extern crate quote;

#[cfg(any(feature = "derive_ends"             ,feature = "attr_ends"))]              mod enum_ends;
#[cfg(any(feature = "derive_field_structs"    ,feature = "attr_field_structs"))]     mod enum_field_structs;
#[cfg(any(feature = "derive_from_discriminant",feature = "attr_from_discriminant"))] mod enum_from_discriminant;
#[cfg(any(feature = "derive_from_index"       ,feature = "attr_from_index"))]        mod enum_from_index;
#[cfg(any(feature = "derive_from"             ,feature = "attr_from"))]              mod enum_from;
#[cfg(any(feature = "derive_from_variant_name",feature = "attr_from_variant_name"))] mod enum_from_variant_name;
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
#[cfg(any(feature = "derive_into"             ,feature = "attr_into"))]              mod enum_into;
mod util;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::ItemEnum;

#[inline(always)]
fn derive_enum<F>(input: TokenStream,gen_derive: F) -> TokenStream
	where F: FnOnce(ItemEnum) -> TokenStream2
{
	let item = syn::parse2::<ItemEnum>(input.into()).expect("`derive(Enum*)` may only be applied to enum items");
	TokenStream::from(gen_derive(item))
}

#[inline(always)]
fn attr_enum<F>(attr: TokenStream,input: TokenStream,gen_derive: F) -> TokenStream
	where F: FnOnce(TokenStream2,ItemEnum) -> TokenStream2
{
	let item = syn::parse::<ItemEnum>(input.into()).expect("`impl_enum_*` may only be applied to enum items");
	TokenStream::from(gen_derive(attr.into(),item))
}

/// Implements [`enum_traits::Len`](../enum_traits/trait.Len.html).
///
/// The length is computed from the number of variants in the enum item.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumLen)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::LEN , 6);
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Len for Enum{
/// 	const LEN: usize = 6;
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_len")]
#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: TokenStream) -> TokenStream{derive_enum(input,enum_len::gen_derive)}

/// Defines a constant assigned to the total number of variants in the enum.
///
/// The length is computed from the number of variants in the enum item.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
///
/// # Syntax
/// `#[impl_enum_len(<OuterAttribute*> <Visibility?> const? <Identifier>)]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_len(LENGTH)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::LENGTH , 6);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const LEN: usize = 6;
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_len")]
#[proc_macro_attribute]
pub fn impl_enum_len(attr: TokenStream,item: TokenStream) -> TokenStream{
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
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumEnds,Debug,PartialEq,Eq)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::FIRST , Enum::A);
/// assert_eq!(Enum::LAST  , Enum::F);
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Ends for Enum{
/// 	const FIRST: Self = Enum::A;
/// 	const LAST: Self = Enum::F;
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_ends")]
#[proc_macro_derive(EnumEnds)]
pub fn derive_EnumEnds(input: TokenStream) -> TokenStream{derive_enum(input,enum_ends::gen_derive)}

/// Defines a constant assigned to the first variant in the enum.
///
/// The ends are computed by using the first variant of the enum in the defined order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum has at least one variant.
/// - The enum's first variant is a unit variant.
///
/// # Syntax
/// `#[impl_enum_first(<OuterAttribute*> <Visibility?> const? <Identifier>)]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,PartialEq,Eq)]
/// #[impl_enum_first(START)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::START , Enum::A);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const START: Self = Enum::A;
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_ends")]
#[proc_macro_attribute]
pub fn impl_enum_first(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_ends::gen_attr_first)
}

/// Defines a constant assigned to the last variant in the enum.
///
/// The ends are computed by using the last variant of the enum in the defined order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum has at least one variant.
/// - The enum's last variant is a unit variant.
///
/// # Syntax
/// `#[impl_enum_last(<OuterAttribute*> <Visibility?> const? <Identifier>)]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,PartialEq,Eq)]
/// #[impl_enum_last(END)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::END , Enum::F);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const END: Self = Enum::F;
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_ends")]
#[proc_macro_attribute]
pub fn impl_enum_last(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_ends::gen_attr_last)
}

/// Implements [`enum_traits::ToIndex`](../enum_traits/trait.ToIndex.html).
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
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
///
/// assert_eq!(Enum::A.index()       , 0);
/// assert_eq!(Enum::B(0).index()    , 1);
/// assert_eq!(Enum::C{c: 1}.index() , 2);
/// assert_eq!(Enum::D.index()       , 3);
/// assert_eq!(Enum::E(2).index()    , 4);
/// assert_eq!(Enum::F{f: 3}.index() , 5);
/// ```
/// automatically expands to the following:
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_to_index")]
#[proc_macro_derive(EnumToIndex)]
pub fn derive_EnumToIndex(input: TokenStream) -> TokenStream{derive_enum(input,enum_to_index::gen_derive)}

/// Defines a function that maps the variants to their index in the defined order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
///
/// # Syntax
/// `#[impl_enum_to_index(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> )]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[impl_enum_to_index(const fn get_index(&self) -> u8)]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// assert_eq!(Enum::A.get_index()       , 0);
/// assert_eq!(Enum::B(0).get_index()    , 1);
/// assert_eq!(Enum::C{c: 1}.get_index() , 2);
/// assert_eq!(Enum::D.get_index()       , 3);
/// assert_eq!(Enum::E(2).get_index()    , 4);
/// assert_eq!(Enum::F{f: 3}.get_index() , 5);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// impl Enum{
/// 	const fn get_index(&self) -> u8{
/// 		match self{
/// 			Enum::A     => 0,
/// 			Enum::B(..) => 1,
/// 			Enum::C{..} => 2,
/// 			Enum::D     => 3,
/// 			Enum::E(..) => 4,
/// 			Enum::F{..} => 5,
/// 		}
/// 	}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_to_index")]
#[proc_macro_attribute]
pub fn impl_enum_to_index(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_to_index::gen_attr)
}

/// Implements [`enum_traits::FromIndex`](../enum_traits/trait.FromIndex.html).
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromIndex,EnumIndex,Debug,Eq,PartialEq)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::from_index(0) , Some(Enum::A));
/// assert_eq!(Enum::from_index(1) , Some(Enum::B));
/// assert_eq!(Enum::from_index(2) , Some(Enum::C));
/// assert_eq!(Enum::from_index(3) , Some(Enum::D));
/// assert_eq!(Enum::from_index(4) , Some(Enum::E));
/// assert_eq!(Enum::from_index(5) , Some(Enum::F));
/// assert_eq!(Enum::from_index(6) , None);
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl Index for Enum{
/// 	type Type = u8;
/// }
/// impl FromIndex for Enum {
/// 	fn from_index(index: <Self as Index>::Type) -> Option<Self>{
/// 		Some(match index{
/// 			 0 => Enum::A,
/// 			 1 => Enum::B,
/// 			 2 => Enum::C,
/// 			 3 => Enum::D,
/// 			 4 => Enum::E,
/// 			 5 => Enum::F,
/// 			 _ => return None,
/// 		})
/// 	}
/// 	unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self{
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_from_index")]
#[proc_macro_derive(EnumFromIndex)]
pub fn derive_EnumFromIndex(input: TokenStream) -> TokenStream{derive_enum(input,enum_from_index::gen_derive)}

#[cfg(feature = "attr_from_index")]
#[proc_macro_attribute]
pub fn impl_enum_from_index(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_from_index::gen_attr_optional)
}

#[cfg(feature = "attr_from_index")]
#[proc_macro_attribute]
pub fn impl_enum_from_index_default(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_from_index::gen_attr_default)
}

/// Implements [`enum_traits::Index`](../enum_traits/trait.Index.html).
///
/// [`enum_traits::Index::Type`](../enum_traits/trait.Index.html#associatedtype.Type) is computed by the smallest unsigned integer type fitting the number of variants minus one.
///
/// # Requirements
///
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
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
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_index")]
#[proc_macro_derive(EnumIndex)]
pub fn derive_EnumIndex(input: TokenStream) -> TokenStream{
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
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumIterable)]
/// enum Enum{A,B,C,D,E,F}
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_iterable")]
#[proc_macro_derive(EnumIterable)]
pub fn derive_EnumIterable(input: TokenStream) -> TokenStream{derive_enum(input,enum_iterable::gen_derive)}

/// Implements [`core::iter::Iterator`].
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum variants are all unit variants.
#[cfg(feature = "derive_iterator")]
#[proc_macro_derive(EnumIterator)]
pub fn derive_EnumIterator(input: TokenStream) -> TokenStream{derive_enum(input,enum_iterator::gen_derive)}

/// Implements [`enum_traits::EnumVariantName`](../enum_traits/trait.VariantName.html).
///
/// The names are generated from the variant names.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
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
///
/// assert_eq!(Enum::A.variant_name()       , "A");
/// assert_eq!(Enum::B(0).variant_name()    , "B");
/// assert_eq!(Enum::C{c: 1}.variant_name() , "C");
/// assert_eq!(Enum::D.variant_name()       , "D");
/// assert_eq!(Enum::E(2).variant_name()    , "E");
/// assert_eq!(Enum::F{f: 3}.variant_name() , "F");
/// ```
/// automatically expands to the following:
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_variant_name")]
#[proc_macro_derive(EnumVariantName)]
pub fn derive_EnumVariantName(input: TokenStream) -> TokenStream {derive_enum(input,enum_variant_name::gen_derive)}

/// Defines a function that maps the variants to their defined name.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
///
/// # Syntax
/// `#[impl_enum_variant_name(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> )]`
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[impl_enum_variant_name(const fn name(&self) -> &'static str)]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// assert_eq!(Enum::A.name()       , "A");
/// assert_eq!(Enum::B(0).name()    , "B");
/// assert_eq!(Enum::C{c: 1}.name() , "C");
/// assert_eq!(Enum::D.name()       , "D");
/// assert_eq!(Enum::E(2).name()    , "E");
/// assert_eq!(Enum::F{f: 3}.name() , "F");
/// ```
/// automatically expands to the following:
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_variant_name")]
#[proc_macro_attribute]
pub fn impl_enum_variant_name(attr: TokenStream,item: TokenStream) -> TokenStream{
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
/// #[derive(EnumFromVariantName,Debug,PartialEq)]
/// enum Enum {
/// 	A,
/// 	B(i32),
/// 	C{speed: f32},
/// 	D,
/// 	E,
/// }
///
/// assert_eq!(Enum::from_str("A") , Ok(Enum::A));
/// assert_eq!(Enum::from_str("B") , Err(()));
/// assert_eq!(Enum::from_str("C") , Err(()));
/// assert_eq!(Enum::from_str("D") , Ok(Enum::D));
/// assert_eq!(Enum::from_str("E") , Ok(Enum::E));
/// assert_eq!(Enum::from_str("F") , Err(()));
/// ```
/// automatically expands to the following:
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
#[cfg(feature = "derive_from_variant_name")]
#[proc_macro_derive(EnumFromVariantName)]
pub fn derive_EnumFromVariantName(input: TokenStream) -> TokenStream {derive_enum(input,enum_from_variant_name::gen_derive)}

/// Defines a mapping from the variants' defined name to its variant.
/// Strings that does not match any of the defined names is mapped to the specified default value.
///
/// Note that only unit variants will be constructable.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
///
/// # Syntax
/// `#[impl_enum_from_variant_name(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> )]`
///
/// # Example
///
///	```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_from_variant_name(from_name)]
/// enum Enum {
/// 	A,
/// 	B(i32),
/// 	C{speed: u8},
/// 	D,
/// 	E,
/// }
///
/// assert_eq!(Enum::from_name("A") , Some(Enum::A));
/// assert_eq!(Enum::from_name("B") , None);
/// assert_eq!(Enum::from_name("C") , None);
/// assert_eq!(Enum::from_name("D") , Some(Enum::D));
/// assert_eq!(Enum::from_name("E") , Some(Enum::E));
/// assert_eq!(Enum::from_name("F") , None);
/// ```
/// automatically expands to the following:
///	```rust
/// enum Enum {
/// 	A,
/// 	B(i32),
/// 	C{speed: u8},
/// 	D,
/// 	E,
/// }
///
/// impl Enum{
///		fn from_name(str: &str) -> Option<Self>{
///			Some(match str{
///				"A" => Enum::A,
///				"D" => Enum::D,
///				"E" => Enum::E,
///				_ => return None
///			})
///		}
///	}
///
/// // <rest is omitted>
///	```
#[cfg(feature = "attr_from_variant_name")]
#[proc_macro_attribute]
pub fn impl_enum_from_variant_name(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_from_variant_name::gen_attr)
}

/// Defines a mapping from the variants' defined name to its variant.
///
/// If the function receives an invalid string (not the name of any of the variants),
/// then the specified default value will be returned.
/// The default value argument in the attribute can make use of the variable `__str` to refer to the input string of the function.
///
/// Note that only unit variants will be constructable.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
///
/// # Syntax
/// `#[impl_enum_from_variant_name_default(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> )]`
///
/// # Example
///
///	```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_from_variant_name_default(from_name,Enum::B(String::from(__str)))]
/// enum Enum {
/// 	A,
/// 	B(String),
/// 	C{speed: u8},
/// 	D,
/// 	E,
/// }
///
/// assert_eq!(Enum::from_name("A") , Enum::A);
/// assert_eq!(Enum::from_name("B") , Enum::B(String::from("B")));
/// assert_eq!(Enum::from_name("C") , Enum::B(String::from("C")));
/// assert_eq!(Enum::from_name("D") , Enum::D);
/// assert_eq!(Enum::from_name("E") , Enum::E);
/// assert_eq!(Enum::from_name("F") , Enum::B(String::from("F")));
/// ```
/// automatically expands to the following:
///	```rust
/// enum Enum {
/// 	A,
/// 	B(String),
/// 	C{speed: u8},
/// 	D,
/// 	E,
/// }
///
/// impl Enum{
///		fn from_name(__str: &str) -> Self{
///			match __str{
///				"A" => Enum::A,
///				"D" => Enum::D,
///				"E" => Enum::E,
///				_ => Enum::B(String::from(__str))
///			}
///		}
///	}
///
/// // <rest is omitted>
///	```
#[cfg(feature = "attr_from_variant_name")]
#[proc_macro_attribute]
pub fn impl_enum_from_variant_name_default(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_from_variant_name::gen_attr_default)
}

/// Implements [`enum_traits::Tag`](../enum_traits/trait.Tag.html).
///
/// If there is no [`enum_tag`](fn@enum_tag) attribute on the derived enum, an enum named `<name of Self> + "Tag"` with only unit variants is automatically created from the derived enum.
/// The unit variant enum will be assigned to [`enum_traits::Tag::Tag`](../enum_traits/trait.Tag.html#associatedtype.Tag) when implementing [`enum_traits::Tag`](../enum_traits/trait.Tag.html) for `Self` with the following defaults:
/// - The default attributes on the tag enum is: `#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]`).
/// - The default visibility of the tag enum is the same as the original enum.
/// - The default name of the tag enum is `<name of Self> + "Tag"`.
///
/// # Arguments
/// The optional supplemental attribute [`enum_tag`](fn@enum_tag) can specify options in the generated enum.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
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
/// #[enum_tag(name(#[deprecated] pub(crate) Enum2HasACustomName))]
/// enum Enum2{
/// 	A,
/// 	B
/// }
///
/// assert_eq!(Enum::A.tag()       , EnumTag::A);
/// assert_eq!(Enum::B(0).tag()    , EnumTag::B);
/// assert_eq!(Enum::C{c: 1}.tag() , EnumTag::C);
/// assert_eq!(Enum::D.tag()       , EnumTag::D);
/// assert_eq!(Enum::E(2).tag()    , EnumTag::E);
/// assert_eq!(Enum::F{f: 3}.tag() , EnumTag::F);
/// ```
/// automatically expands to the following:
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_tag")]
#[proc_macro_derive(EnumTag)]
pub fn derive_EnumTag(input: TokenStream) -> TokenStream{derive_enum(input,enum_tag::gen_derive)}

/// Defines an enum with the same variants, but without any fields.
///
/// # Arguments
///
/// The accepted arguments are a comma-separated list of the following fields:
/// - `name(<OuterAttribute*> <Visibility?> enum? <Identifier> )`:
///
///   Generates an enum item with only unit variants based on the original enum.
///
/// - `to(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> )`:
///
///   Generates a function from the original enum to the previously generated enum using the `name` argument.
///
/// - `is(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> )`:
///
///   Generates a function that checks if a value of the original enum is a certain variant from the previously generated enum using the `name` argument.
///
/// # Example 1
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[enum_tag(name(#[derive(Debug,Eq,PartialEq)] pub Enum2),to(pub fn tag))]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// assert_eq!(Enum::A.tag()      ,Enum2::A);
/// assert_eq!(Enum::B(3).tag()   ,Enum2::B);
/// assert_eq!(Enum::C{c: 4}.tag(),Enum2::C);
/// assert_eq!(Enum::D.tag()      ,Enum2::D);
/// assert_eq!(Enum::E(5).tag()   ,Enum2::E);
/// assert_eq!(Enum::F{f: 6}.tag(),Enum2::F);
/// ```
/// automatically expands to the following:
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
/// #[derive(Debug,Eq,PartialEq)]
/// pub enum Enum2{A,B,C,D,E,F}
///
/// impl Enum{
/// 	pub fn tag(&self) -> Enum2{match self{
/// 		Enum::A     => Enum2::A,
/// 		Enum::B(..) => Enum2::B,
/// 		Enum::C{..} => Enum2::C,
/// 		Enum::D     => Enum2::D,
/// 		Enum::E(..) => Enum2::E,
/// 		Enum::F{..} => Enum2::F,
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
///
/// # Example 2
///
/// ```rust
/// #![feature(min_adt_const_params)]
/// use core::marker::ConstParamTy;
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[enum_tag(name(#[derive(ConstParamTy,PartialEq,Eq)] pub Enum2),is(pub fn is))]
/// enum Enum{
/// 	A,
/// 	B(u8),
/// 	C{c: u16},
/// 	D,
/// 	E(u32),
/// 	F{f: u64},
/// }
///
/// assert!(Enum::A.is::<{Enum2::A}>());
/// assert!(Enum::B(3).is::<{Enum2::B}>());
/// assert!(Enum::C{c: 4}.is::<{Enum2::C}>());
/// assert!(Enum::D.is::<{Enum2::D}>());
/// assert!(Enum::E(5).is::<{Enum2::E}>());
/// assert!(Enum::F{f: 6}.is::<{Enum2::F}>());
/// ```
/// automatically expands to the following:
/// ```rust
/// #![feature(min_adt_const_params)]
/// use core::marker::ConstParamTy;
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
/// #[derive(ConstParamTy,PartialEq,Eq)]
/// pub enum Enum2{A,B,C,D,E,F}
///
/// impl Enum{
/// 	pub fn is<const e: Enum2>(&self) -> bool{match e{
/// 		Enum2::A => if let Enum::A     = self{true}else{false},
/// 		Enum2::B => if let Enum::B(..) = self{true}else{false},
/// 		Enum2::C => if let Enum::C{..} = self{true}else{false},
/// 		Enum2::D => if let Enum::D     = self{true}else{false},
/// 		Enum2::E => if let Enum::E(..) = self{true}else{false},
/// 		Enum2::F => if let Enum::F{..} = self{true}else{false},
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_tag")]
#[proc_macro_attribute]
pub fn enum_tag(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_tag::gen_attr)
}

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
/// enum Enum2{
/// 	A,
/// 	#[enum_is(name(#[deprecated] pub(crate) custom_name))] B
/// }
/// ```
/// automatically expands to the following:
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
///     fn is_a(&self) -> bool{
///         if let &Enum2::A = self {true} else {false}
///     }
///     #[deprecated] pub(crate) fn custom_name(&self) -> bool{
///         if let &Enum2::B = self {true} else {false}
///     }
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_is")]
#[proc_macro_derive(EnumIs,attributes(enum_is))]
pub fn derive_EnumIs(input: TokenStream) -> TokenStream{derive_enum(input,enum_is::gen_derive)}

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
/// automatically expands to the following:
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
#[proc_macro_derive(EnumFrom,attributes(enum_from))]
pub fn derive_EnumFrom(input: TokenStream) -> TokenStream{derive_enum(input,enum_from::gen_derive)}

/// Implements [`enum_traits::Step`](../enum_traits/trait.Step.html).
///
/// The next and previous functions are computed by using the variants of the enum in their defined order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum's variants are all unit variants.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[derive(EnumStep)]
/// enum Enum{A,B,C,D}
///
/// assert_eq!(Enum::A.previous() , None);
/// assert_eq!(Enum::B.previous() , Some(Enum::A));
/// assert_eq!(Enum::C.previous() , Some(Enum::B));
/// assert_eq!(Enum::D.previous() , Some(Enum::C));
///
/// assert_eq!(Enum::A.next() , Some(Enum::B));
/// assert_eq!(Enum::B.next() , Some(Enum::C));
/// assert_eq!(Enum::C.next() , Some(Enum::D));
/// assert_eq!(Enum::D.next() , None);
/// ```
/// automatically expands to the following:
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
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_step")]
#[proc_macro_derive(EnumStep)]
pub fn derive_EnumStep(input: TokenStream) -> TokenStream{derive_enum(input,enum_step::gen_derive)}

/// Defines a function that maps the variants to their previous variant in the defined order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_prev(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?>)]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_prev(const fn prev_variant(self) -> Option<Self>)]
/// enum Enum{A,B,C,D}
///
/// assert_eq!(Enum::A.prev_variant() , None);
/// assert_eq!(Enum::B.prev_variant() , Some(Enum::A));
/// assert_eq!(Enum::C.prev_variant() , Some(Enum::B));
/// assert_eq!(Enum::D.prev_variant() , Some(Enum::C));
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D}
///
/// impl Enum{
/// 	const fn prev_variant(self) -> Option<Self>{match self{
/// 		Enum::A => None,
/// 		Enum::B => Some(Enum::A),
/// 		Enum::C => Some(Enum::B),
/// 		Enum::D => Some(Enum::C),
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_step")]
#[proc_macro_attribute]
pub fn impl_enum_prev(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_step::gen_attr_prev)
}

/// Defines a function that maps the variants to their next variant in the defined order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_next(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> )]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_next(const fn next_variant(self) -> Option<Self>)]
/// enum Enum{A,B,C,D}
///
/// assert_eq!(Enum::A.next_variant() , Some(Enum::B));
/// assert_eq!(Enum::B.next_variant() , Some(Enum::C));
/// assert_eq!(Enum::C.next_variant() , Some(Enum::D));
/// assert_eq!(Enum::D.next_variant() , None);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D}
///
/// impl Enum{
/// 	const fn next_variant(self) -> Option<Self>{match self{
/// 		Enum::A => Some(Enum::B),
/// 		Enum::B => Some(Enum::C),
/// 		Enum::C => Some(Enum::D),
/// 		Enum::D => None,
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_step")]
#[proc_macro_attribute]
pub fn impl_enum_next(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_step::gen_attr_next)
}

/// Defines a function that maps the variants to their next variant in the defined order.
/// The first variant is mapped to the specified default value.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_prev_default(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> , <Expr> )]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_prev_default(const fn prev_variant(self) -> Self , Enum::D)]
/// enum Enum{A,B,C,D}
///
/// assert_eq!(Enum::A.prev_variant() , Enum::D);
/// assert_eq!(Enum::B.prev_variant() , Enum::A);
/// assert_eq!(Enum::C.prev_variant() , Enum::B);
/// assert_eq!(Enum::D.prev_variant() , Enum::C);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D}
///
/// impl Enum{
/// 	const fn prev_variant(self) -> Self{match self{
/// 		Enum::A => Enum::D,
/// 		Enum::B => Enum::A,
/// 		Enum::C => Enum::B,
/// 		Enum::D => Enum::C,
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_step")]
#[proc_macro_attribute]
pub fn impl_enum_prev_default(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_step::gen_attr_prev_default)
}

/// Defines a function that maps the variants to their next variant in the defined order.
/// The last variant is mapped to the specified default value.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_next_default(<OuterAttribute*> <Visibility?> <FunctionQualifiers?> fn? <Identifier> <GenericParams?> ( <FunctionParameters?> ) <FunctionReturnType?> , <Expr> )]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_next_default(const fn next_variant(self) -> Self , Enum::A)]
/// enum Enum{A,B,C,D}
///
/// assert_eq!(Enum::A.next_variant() , Enum::B);
/// assert_eq!(Enum::B.next_variant() , Enum::C);
/// assert_eq!(Enum::C.next_variant() , Enum::D);
/// assert_eq!(Enum::D.next_variant() , Enum::A);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D}
///
/// impl Enum{
/// 	const fn next_variant(self) -> Self{match self{
/// 		Enum::A => Enum::B,
/// 		Enum::B => Enum::C,
/// 		Enum::C => Enum::D,
/// 		Enum::D => Enum::A,
/// 	}}
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_step")]
#[proc_macro_attribute]
pub fn impl_enum_next_default(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_step::gen_attr_next_default)
}

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
///
/// impl_IntoDiscriminant_of_numeric!(u8,Enum);
///
/// assert_eq!(Enum::from_discriminant(8)   , Some(Enum::A));
/// assert_eq!(Enum::from_discriminant(5)   , Some(Enum::C));
/// assert_eq!(Enum::from_discriminant(7)   , Some(Enum::D));
/// assert_eq!(Enum::from_discriminant(200) , None);
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
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
pub fn derive_EnumFromDiscriminant(input: TokenStream) -> TokenStream{derive_enum(input,enum_from_discriminant::gen_derive)}

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
/// # Example
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
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_field_structs")]
#[proc_macro_derive(EnumFieldStructs,attributes(enum_field_structs))]
pub fn derive_EnumFieldStructs(input: TokenStream) -> TokenStream{derive_enum(input,enum_field_structs::gen_derive)}

/// Implements [`enum_traits::VariantsArray`](../enum_traits/trait.VariantsArray.html).
///
/// The array is constructed by the variants in order.
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum's variants are all unit variants.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumVariantsArray,Debug,Eq,PartialEq)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::VARIANTS , &[Enum::A,Enum::B,Enum::C,Enum::D,Enum::E,Enum::F]);
/// ```
/// automatically expands to the following:
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C,D,E,F}
///
/// impl VariantsArray for Enum{
/// 	const VARIANTS: &'static [Self] = &[Self::A,Self::B,Self::C,Self::D,Self::E,Self::F];
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "derive_variants_array")]
#[proc_macro_derive(EnumVariantsArray)]
pub fn derive_EnumVariantsArray(input: TokenStream) -> TokenStream{derive_enum(input,enum_variants_array::gen_derive)}

/// Defines an array constant that lists every variant of the enum in order.
///
/// # Requirements
/// - The attribute must be applied to an enum item.
/// - The enum's variants are all unit variants.
///
/// # Syntax
/// `#[impl_enum_variants_array(<OuterAttribute*> <Visibility?> const? <Identifier>)]`
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(Debug,Eq,PartialEq)]
/// #[impl_enum_variants_array(VARIANTS)]
/// enum Enum{A,B,C,D,E,F}
///
/// assert_eq!(Enum::VARIANTS , [Enum::A,Enum::B,Enum::C,Enum::D,Enum::E,Enum::F]);
/// ```
/// automatically expands to the following:
/// ```rust
/// enum Enum{A,B,C,D,E,F}
///
/// impl Enum{
/// 	const VARIANTS: [Self; 6] = [Enum::A,Enum::B,Enum::C,Enum::D,Enum::E,Enum::F];
/// }
///
/// // <rest is omitted>
/// ```
#[cfg(feature = "attr_variants_array")]
#[proc_macro_attribute]
pub fn impl_enum_variants_array(attr: TokenStream,item: TokenStream) -> TokenStream{
	attr_enum(attr,item,enum_variants_array::gen_attr)
}

#[cfg(feature = "derive_into")]
#[proc_macro_derive(EnumInto)]
pub fn derive_EnumInto(input: TokenStream) -> TokenStream{derive_enum(input,enum_into::gen_derive)}
