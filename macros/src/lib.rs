//! Derives and procedural macros for enum items.
//!
//! Also see [`enum_traits`] for some of the traits that this library derives.

#![allow(non_snake_case)]
#![no_std]

extern crate alloc;
#[macro_use] extern crate quote;

mod enum_ends;
mod enum_from_index;
mod enum_from_variant_name;
mod enum_from_variant_type;
mod enum_index;
mod enum_is_variant_fns;
mod enum_iter;
mod enum_iterator;
mod enum_len;
mod enum_tag;
mod enum_to_index;
mod enum_variant_name;
mod util;

use proc_macro2::TokenStream;
use syn::ItemEnum;

#[inline(always)]
fn derive_enum<F>(input: proc_macro::TokenStream,gen_impl: F) -> proc_macro::TokenStream
	where F: FnOnce(ItemEnum) -> TokenStream
{
	let input = proc_macro2::TokenStream::from(input);
	let item = syn::parse2::<ItemEnum>(input).expect("`derive(Enum*)` may only be applied to enum items");
	proc_macro::TokenStream::from(gen_impl(item))
}

/// Implements [`enum_traits::Len`].
///
/// The length is computed from the number of variants in the enum item.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Examples
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(EnumLen)]enum T{}
/// 	assert_eq!(0,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A}
/// 	assert_eq!(1,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C}
/// 	assert_eq!(3,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G}
/// 	assert_eq!(7,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H}
/// 	assert_eq!(8,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
/// 	assert_eq!(25,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{
/// 		A,
/// 		B(),
/// 		C{},
/// 		D(u8),
/// 		E{e: u8},
/// 		F(u8,u16),
/// 		G{g1: u8,g2: u16},
/// 		H
/// 	}
/// 	assert_eq!(8,T::LEN);
/// }
/// ```
#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_len::gen_impl)}

/// Implements [`enum_traits::Ends`].
///
/// The ends are computed by using the first and the last variant of the enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum has at least one variant
/// - The enum's first variant is an unit variant
/// - The enum's last variant is an unit variant
///
/// # Examples
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::A , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A,B}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::B , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A,B,C}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::C , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::G , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G,H}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::H , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::Z , T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]
/// 	enum T{
/// 		A,
/// 		B(),
/// 		C{},
/// 		D(u8),
/// 		E{e: u8},
/// 		F(u8,u16),
/// 		G{g1: u8,g2: u16},
/// 		H
/// 	}
///
/// 	assert_eq!(T::A , T::first());
/// 	assert_eq!(T::H , T::last());
/// }
/// ```
#[proc_macro_derive(EnumEnds)]
pub fn derive_EnumEnds(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_ends::gen_impl)}

/// Implements [`enum_traits::ToIndex`].
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(EnumIndex,EnumToIndex)]
/// 	enum T{}
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]
/// 	enum T{A}
///
/// 	assert_eq!(0,T::A.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]
/// 	enum T{A,B,C,D,E,F,G,H}
///
/// 	assert_eq!(0,T::A.index());
/// 	assert_eq!(1,T::B.index());
/// 	assert_eq!(2,T::C.index());
/// 	assert_eq!(3,T::D.index());
/// 	assert_eq!(4,T::E.index());
/// 	assert_eq!(5,T::F.index());
/// 	assert_eq!(6,T::G.index());
/// 	assert_eq!(7,T::H.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// 	assert_eq!(1,T::B.into_index());
/// 	assert_eq!(2,T::C.into_index());
/// 	assert_eq!(3,T::D.into_index());
/// 	assert_eq!(4,T::E.into_index());
/// 	assert_eq!(5,T::F.into_index());
/// 	assert_eq!(6,T::G.into_index());
/// 	assert_eq!(7,T::H.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]
/// 	enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
///
/// 	assert_eq!(00,T::A.index());
/// 	assert_eq!(01,T::B.index());
/// 	assert_eq!(02,T::C.index());
/// 	assert_eq!(03,T::D.index());
/// 	assert_eq!(04,T::E.index());
/// 	assert_eq!(05,T::F.index());
/// 	assert_eq!(06,T::G.index());
/// 	assert_eq!(07,T::H.index());
/// 	assert_eq!(08,T::I.index());
/// 	assert_eq!(09,T::J.index());
/// 	assert_eq!(10,T::K.index());
/// 	assert_eq!(11,T::L.index());
/// 	assert_eq!(12,T::M.index());
/// 	assert_eq!(13,T::N.index());
/// 	assert_eq!(14,T::O.index());
/// 	assert_eq!(15,T::P.index());
/// 	assert_eq!(16,T::Q.index());
/// 	assert_eq!(17,T::R.index());
/// 	assert_eq!(18,T::S.index());
/// 	assert_eq!(19,T::T.index());
/// 	assert_eq!(20,T::U.index());
/// 	assert_eq!(21,T::V.index());
/// 	assert_eq!(22,T::X.index());
/// 	assert_eq!(23,T::Y.index());
/// 	assert_eq!(24,T::Z.index());
///
/// 	assert_eq!(00,T::A.into_index());
/// 	assert_eq!(01,T::B.into_index());
/// 	assert_eq!(02,T::C.into_index());
/// 	assert_eq!(03,T::D.into_index());
/// 	assert_eq!(04,T::E.into_index());
/// 	assert_eq!(05,T::F.into_index());
/// 	assert_eq!(06,T::G.into_index());
/// 	assert_eq!(07,T::H.into_index());
/// 	assert_eq!(08,T::I.into_index());
/// 	assert_eq!(09,T::J.into_index());
/// 	assert_eq!(10,T::K.into_index());
/// 	assert_eq!(11,T::L.into_index());
/// 	assert_eq!(12,T::M.into_index());
/// 	assert_eq!(13,T::N.into_index());
/// 	assert_eq!(14,T::O.into_index());
/// 	assert_eq!(15,T::P.into_index());
/// 	assert_eq!(16,T::Q.into_index());
/// 	assert_eq!(17,T::R.into_index());
/// 	assert_eq!(18,T::S.into_index());
/// 	assert_eq!(19,T::T.into_index());
/// 	assert_eq!(20,T::U.into_index());
/// 	assert_eq!(21,T::V.into_index());
/// 	assert_eq!(22,T::X.into_index());
/// 	assert_eq!(23,T::Y.into_index());
/// 	assert_eq!(24,T::Z.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]
/// 	enum T{
/// 		A,
/// 		B(),
/// 		C{},
/// 		D(u8),
/// 		E{e: u8},
/// 		F(u8,u16),
/// 		G{g1: u8,g2: u16},
/// 		H
/// 	}
///
/// 	assert_eq!(0,T::A.index());
/// 	assert_eq!(1,T::B().index());
/// 	assert_eq!(2,T::C{}.index());
/// 	assert_eq!(3,T::D(0).index());
/// 	assert_eq!(4,T::E{e: 0}.index());
/// 	assert_eq!(5,T::F(0,0).index());
/// 	assert_eq!(6,T::G{g1: 0,g2: 0}.index());
/// 	assert_eq!(7,T::H.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// 	assert_eq!(1,T::B().into_index());
/// 	assert_eq!(2,T::C{}.into_index());
/// 	assert_eq!(3,T::D(0).into_index());
/// 	assert_eq!(4,T::E{e: 0}.into_index());
/// 	assert_eq!(5,T::F(0,0).into_index());
/// 	assert_eq!(6,T::G{g1: 0,g2: 0}.into_index());
/// 	assert_eq!(7,T::H.into_index());
/// }
/// ```
#[proc_macro_derive(EnumToIndex)]
pub fn derive_EnumToIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_to_index::gen_impl)}

/// Implements [`enum_traits::FromIndex`].
///
/// A variant's index is computed by its index in the defined order of the enum item.
///
/// # Requirements
/// - The derived item is an enum
#[proc_macro_derive(EnumFromIndex)]
pub fn derive_EnumFromIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_from_index::gen_impl)}

/// Implements [`enum_traits::Index`].
///
/// [`Type`][`enum_traits::Index::Type`] is computed by the following:
/// - If the `repr` attribute exists for the enum item, it becomes the type specified in `repr`.
/// - Else, the smallest integer type fitting the number of variants is used.
///
/// # Requirements
///
/// - The derived item is an enum
#[proc_macro_derive(EnumIndex)]
pub fn derive_EnumIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	derive_enum(input,enum_index::gen_impl)
}

/// Creates a struct and implements [`enum_traits::Iterable`].
///
/// A struct named ((name of Self) + "Iter") will be generated with the same visibility as `Self`.
/// This struct will then implement `Iterator` and `Iter` will be assigned to it when implementing `Iterable` for `Self`.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum variants are all unit variants
#[proc_macro_derive(EnumIter)]
pub fn derive_EnumIter(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_iter::gen_impl)}

/// Implements [`Iterator`].
///
/// # Requirements
/// - The derived item is an enum
/// - The enum variants are all unit variants
#[proc_macro_derive(EnumIterator)]
pub fn derive_EnumIterator(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_iterator::gen_impl)}

/// Implements [`enum_traits::EnumVariantName`].
///
/// The names are generated from the variant names.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Examples
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumVariantName)]
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
///
/// assert_eq!(Enum::Dog.variant_name(), "Dog");
/// assert_eq!(Enum::Cat(0).variant_name(), "Cat");
/// assert_eq!(Enum::Robot{speed: 0.0}.variant_name(), "Robot");
/// ```
#[proc_macro_derive(EnumVariantName)]
pub fn derive_EnumVariantName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {derive_enum(input,enum_variant_name::gen_impl)}

/// Implements [`core::str::FromStr`].
///
/// # Requirements
/// - The derived item is an enum.
/// - The enum variants are all unit variants.
///
/// # Example:
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
/// assert_eq!(Ok(Enum::A) , Enum::from_str("A"));
/// assert_eq!(Err(())     , Enum::from_str("B"));
/// assert_eq!(Err(())     , Enum::from_str("C"));
/// assert_eq!(Ok(Enum::D) , Enum::from_str("D"));
/// assert_eq!(Ok(Enum::E) , Enum::from_str("E"));
/// ```
///
/// # Expanded:
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
///	```
#[proc_macro_derive(EnumFromVariantName)]
pub fn derive_EnumFromVariantName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {derive_enum(input,enum_from_variant_name::gen_impl)}

/// Creates an enum with unit variants from the derived enum, and implements [`enum_traits::Tag`].
///
/// An enum named ((name of Self) + "Tag") will be created with the same visibility as `Self`.
/// This enum will then be assigned to [`enum_traits::Tag::Tag`] when implementing [`enum_traits::Tag`] for `Self`.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Examples
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumTag)]
/// enum Enum{
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
///
/// assert_eq!(EnumTag::Dog  ,Enum::Dog.tag());
/// assert_eq!(EnumTag::Cat  ,Enum::Cat(0).tag());
/// assert_eq!(EnumTag::Robot,Enum::Robot{speed: 0.0}.tag());
/// ```
///
/// Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
///
/// enum EnumTag{
/// 	Dog,
/// 	Cat,
/// 	Robot,
/// }
///
/// impl Tag for Enum{
/// 	type Tag = EnumTag;
///
/// 	fn into_tag(self) -> Self::Tag{
/// 		match self {
/// 			Enum::Dog       => EnumTag::Dog,
/// 			Enum::Cat(..)   => EnumTag::Cat,
/// 			Enum::Robot{..} => EnumTag::Robot,
/// 		}
/// 	}
///
/// 	fn tag(&self) -> Self::Tag{
/// 		match self {
/// 			&Enum::Dog       => EnumTag::Dog,
/// 			&Enum::Cat(..)   => EnumTag::Cat,
/// 			&Enum::Robot{..} => EnumTag::Robot,
/// 		}
/// 	}
/// }
/// ```
#[proc_macro_derive(EnumTag)]
pub fn derive_EnumTag(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_tag::gen_impl)}

/// Implements functions that checks if an value of the enum matches a certain variant.
///
/// # Requirements
/// - The derived item is an enum.
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumIsVariantFns)]
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
///
/// assert!(Enum::Dog.is_dog());
/// assert!(Enum::Cat(0).is_cat());
/// assert!(Enum::Robot{speed: 0.0}.is_robot());
///
/// assert!(!Enum::Dog.is_cat());
/// assert!(!Enum::Dog.is_robot());
/// assert!(!Enum::Robot{speed: 0.0}.is_cat());
/// assert!(!Enum::Robot{speed: 0.0}.is_dog());
/// assert!(!Enum::Cat(0).is_dog());
/// assert!(!Enum::Cat(0).is_robot());
/// ```
///
/// # Expanded example
///
/// ```rust
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
///
/// impl Enum{
/// 	fn is_dog(&self) -> bool{if let &Enum::Dog = self {true} else {false}}
/// 	fn is_cat(&self) -> bool{if let &Enum::Cat(_) = self {true} else {false}}
/// 	fn is_robot(&self) -> bool{if let &Enum::Robot{..} = self {true} else {false}}
/// }
/// ```

#[proc_macro_derive(EnumIsVariantFns)]
pub fn derive_EnumIsVariantFns(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_is_variant_fns::gen_impl)}

/// Implements [`From`] for all variants in the enum by using the type of the variant.
///
/// If more than one field exist in a variant, a tuple is used to represent the variant (Both `enum Enum{A{x: i8}}` and `enum Enum{A(i8)}` generates `impl From<i8>`).
///
/// If no fields exist in a variant, an empty tuple is used to represent the variant (`enum Enum{A}` generates `impl From<()>`).
///
/// Note that both tuple variants and record variants become tuples in the `impl From`.
///
/// # Requirements
/// - The derived item is an enum.
/// - There are no variants in which their types are identical when all type parameters are applied.
///
/// # Example
///
/// ```rust
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromVariantType,Debug,PartialEq,Eq)]
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
/// ```
#[proc_macro_derive(EnumFromVariantType)]
pub fn derive_EnumFromVariantType(input: proc_macro::TokenStream) -> proc_macro::TokenStream{derive_enum(input,enum_from_variant_type::gen_impl)}
