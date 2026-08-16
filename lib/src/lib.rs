//! Traits for enum items.
//!
//! Also see [`enum_traits_macros`], the crate where the implementations of these traits are automatically generated.
//!
//! These crates together generate various additional functionalities for enums based on their definitions.
//!
//! Note that this library is not required for [`enum_traits_macros`] to function.
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
//!
//! # Examples
//!
//! Enum without fields:
//! ```rust
//! use core::str::FromStr;
//! use enum_traits_macros::*;
//! use enum_traits::*;
//!
//! #[derive(Debug,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumStep,EnumVariantName,EnumFromStr,EnumFromDiscriminant)]
//! enum Enum{
//! 	A = 10,
//! 	B = 2,
//! 	C = 4,
//! 	D,
//! 	E = 16,
//! 	F = 33
//! }
//! impl_IntoDiscriminant_of_numeric!(u8,Enum);
//!
//! //Functions based on a variant's position.
//! assert_eq!(Enum::from_index(0)         , Some(Enum::A));
//! assert_eq!(Enum::B.into_index()        , 1);
//! assert_eq!(Enum::LEN                   , 6);
//! assert_eq!(Enum::LAST                , Enum::F);
//! assert_eq!(Enum::B.next()              , Some(Enum::C));
//!
//! //Functions based on a variant's name.
//! assert_eq!(Enum::D.variant_name()      , "D");
//! assert_eq!(Enum::from_str("E")         , Ok(Enum::E));
//!
//! //Functions based on a variant's discriminant.
//! assert_eq!(Enum::from_discriminant(33) , Some(Enum::F));
//! assert_eq!(
//! 	Enum::from_discriminant(::core::mem::discriminant(&Enum::F)),
//! 	Some(Enum::F)
//! );
//! ```
//!
//! Enum with fields and type parameters:
//! ```rust
//! use core::str::FromStr;
//! use enum_traits_macros::*;
//! use enum_traits::*;
//!
//! #[derive(Debug,PartialEq,EnumIndex,EnumToIndex,EnumLen,EnumVariantName,EnumIs,EnumFrom,EnumTag)]
//! enum Enum<'l,T: ?Sized,const LEN: usize,I> where I: Iterator<Item = T>{
//! 	None,
//! 	List([&'l T; LEN]),
//! 	NonEmpty(&'l T,I),
//! 	Join{first: I , middle: &'l T , last: I},
//! }
//!
//! type E = Enum<'static,u8,3,::core::ops::Range<u8>>;
//!
//! //Functions based on a variant's position.
//! assert_eq!(E::None.into_index()                , 0);
//! assert_eq!(E::LEN                              , 4);
//!
//! //Functions based on a variant's name.
//! assert_eq!(E::NonEmpty(&2,1..2).variant_name() , "NonEmpty");
//!
//! //Boolean checks of variants.
//! assert!(E::List([&1,&2,&3]).is_list());
//! assert!(!E::None.is_non_empty());
//!
//! //Constructor from an enum's fields.
//! assert_eq!(E::from([&1,&2,&3])                 , E::List([&1,&2,&3]));
//! assert_eq!(E::from((&1,2..3))                  , E::NonEmpty(&1,2..3));
//!
//! //Fieldless version of an enum.
//! assert_eq!(E::List([&1,&2,&3]).tag()           , EnumTag::List);
//! assert_eq!(E::NonEmpty(&1,2..3).tag()          , EnumTag::NonEmpty);
//!
//! ```

#![no_std]

use core::mem;

#[cfg(feature = "derive")]
pub use enum_traits_macros::*;

/// Represents the type used for indexing the variants of an enum item type.
///
/// This is primarily used by [`FromIndex`] and [`ToIndex`].
///
/// Derive this trait for an enum automatically using [`#[derive(EnumIndex)]`][enum_traits_macros::EnumIndex].
///
/// # Requirements
///
/// - [`Type`][`Index::Type`] should be a primitive unsigned integer type.
/// - The number of variants of `Self` should be lesser than or equal the number of values of [`Type`][`Index::Type`].
pub trait Index{
	/// Type used as an index for the variants of `Self`.
	type Type;
}

/// A constructor from an index based on an order on the variants of an enum type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumFromIndex)]`][enum_traits_macros::EnumFromIndex].
///
/// # Examples of correctness
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(Debug,PartialEq,EnumIndex,EnumFromIndex)]
/// 	enum T{}
///
/// 	assert_eq!(None,T::from_index(0));
/// }{
/// 	#[derive(Debug,PartialEq,EnumIndex,EnumFromIndex)]
/// 	enum T{A}
///
/// 	assert_eq!(Some(T::A),T::from_index(0));
/// 	assert_eq!(None      ,T::from_index(1));
///
/// 	assert_eq!(T::A,unsafe{T::from_index_unchecked(0)});
/// }{
/// 	#[derive(Debug,PartialEq,EnumIndex,EnumFromIndex)]
/// 	enum T{A=5,B=3,C=2,D=10,E=100,F=1000,G=500,H}
///
/// 	assert_eq!(Some(T::A),T::from_index(0));
/// 	assert_eq!(Some(T::B),T::from_index(1));
/// 	assert_eq!(Some(T::C),T::from_index(2));
/// 	assert_eq!(Some(T::D),T::from_index(3));
/// 	assert_eq!(Some(T::E),T::from_index(4));
/// 	assert_eq!(Some(T::F),T::from_index(5));
/// 	assert_eq!(Some(T::G),T::from_index(6));
/// 	assert_eq!(Some(T::H),T::from_index(7));
/// 	assert_eq!(None      ,T::from_index(8));
///
/// 	assert_eq!(T::A,unsafe{T::from_index_unchecked(0)});
/// 	assert_eq!(T::B,unsafe{T::from_index_unchecked(1)});
/// 	assert_eq!(T::C,unsafe{T::from_index_unchecked(2)});
/// 	assert_eq!(T::D,unsafe{T::from_index_unchecked(3)});
/// 	assert_eq!(T::E,unsafe{T::from_index_unchecked(4)});
/// 	assert_eq!(T::F,unsafe{T::from_index_unchecked(5)});
/// 	assert_eq!(T::G,unsafe{T::from_index_unchecked(6)});
/// 	assert_eq!(T::H,unsafe{T::from_index_unchecked(7)});
/// }{
/// 	#[derive(Debug,PartialEq,EnumIndex,EnumFromIndex)]
/// 	enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
///
/// 	assert_eq!(Some(T::A),T::from_index(00));
/// 	assert_eq!(Some(T::B),T::from_index(01));
/// 	assert_eq!(Some(T::C),T::from_index(02));
/// 	assert_eq!(Some(T::D),T::from_index(03));
/// 	assert_eq!(Some(T::E),T::from_index(04));
/// 	assert_eq!(Some(T::F),T::from_index(05));
/// 	assert_eq!(Some(T::G),T::from_index(06));
/// 	assert_eq!(Some(T::H),T::from_index(07));
/// 	assert_eq!(Some(T::I),T::from_index(08));
/// 	assert_eq!(Some(T::J),T::from_index(09));
/// 	assert_eq!(Some(T::K),T::from_index(10));
/// 	assert_eq!(Some(T::L),T::from_index(11));
/// 	assert_eq!(Some(T::M),T::from_index(12));
/// 	assert_eq!(Some(T::N),T::from_index(13));
/// 	assert_eq!(Some(T::O),T::from_index(14));
/// 	assert_eq!(Some(T::P),T::from_index(15));
/// 	assert_eq!(Some(T::Q),T::from_index(16));
/// 	assert_eq!(Some(T::R),T::from_index(17));
/// 	assert_eq!(Some(T::S),T::from_index(18));
/// 	assert_eq!(Some(T::T),T::from_index(19));
/// 	assert_eq!(Some(T::U),T::from_index(20));
/// 	assert_eq!(Some(T::V),T::from_index(21));
/// 	assert_eq!(Some(T::X),T::from_index(22));
/// 	assert_eq!(Some(T::Y),T::from_index(23));
/// 	assert_eq!(Some(T::Z),T::from_index(24));
/// 	assert_eq!(None      ,T::from_index(25));
///
/// 	assert_eq!(T::A,unsafe{T::from_index_unchecked(00)});
/// 	assert_eq!(T::B,unsafe{T::from_index_unchecked(01)});
/// 	assert_eq!(T::C,unsafe{T::from_index_unchecked(02)});
/// 	assert_eq!(T::D,unsafe{T::from_index_unchecked(03)});
/// 	assert_eq!(T::E,unsafe{T::from_index_unchecked(04)});
/// 	assert_eq!(T::F,unsafe{T::from_index_unchecked(05)});
/// 	assert_eq!(T::G,unsafe{T::from_index_unchecked(06)});
/// 	assert_eq!(T::H,unsafe{T::from_index_unchecked(07)});
/// 	assert_eq!(T::I,unsafe{T::from_index_unchecked(08)});
/// 	assert_eq!(T::J,unsafe{T::from_index_unchecked(09)});
/// 	assert_eq!(T::K,unsafe{T::from_index_unchecked(10)});
/// 	assert_eq!(T::L,unsafe{T::from_index_unchecked(11)});
/// 	assert_eq!(T::M,unsafe{T::from_index_unchecked(12)});
/// 	assert_eq!(T::N,unsafe{T::from_index_unchecked(13)});
/// 	assert_eq!(T::O,unsafe{T::from_index_unchecked(14)});
/// 	assert_eq!(T::P,unsafe{T::from_index_unchecked(15)});
/// 	assert_eq!(T::Q,unsafe{T::from_index_unchecked(16)});
/// 	assert_eq!(T::R,unsafe{T::from_index_unchecked(17)});
/// 	assert_eq!(T::S,unsafe{T::from_index_unchecked(18)});
/// 	assert_eq!(T::T,unsafe{T::from_index_unchecked(19)});
/// 	assert_eq!(T::U,unsafe{T::from_index_unchecked(20)});
/// 	assert_eq!(T::V,unsafe{T::from_index_unchecked(21)});
/// 	assert_eq!(T::X,unsafe{T::from_index_unchecked(22)});
/// 	assert_eq!(T::Y,unsafe{T::from_index_unchecked(23)});
/// 	assert_eq!(T::Z,unsafe{T::from_index_unchecked(24)});
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
/// Derive this trait for an enum automatically using [`#[derive(EnumToIndex)]`][enum_traits_macros::EnumToIndex]
///
/// # Examples of correctness
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
/// 	enum T{A=5,B=3,C=2,D=10,E=100,F=1000,G=500,H}
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
pub trait ToIndex: Index{
	/// Index in the defined order of an enum
	fn into_index(self) -> <Self as Index>::Type;

	/// Index in the defined order of an enum
	fn index(&self) -> <Self as Index>::Type;
}

/// Number of variants in an enum type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumLen)]`][enum_traits_macros::EnumLen].
///
/// # Examples of correctness
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
pub trait Len{
	/// Number of variants in an item.
	const LEN: usize;
}

/// Constructors for the endpoints of an enum based on an order on the variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumEnds)]`][enum_traits_macros::EnumEnds].
///
/// # Examples of correctness
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::A , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A,B}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::B , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A,B,C}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::C , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::G , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G,H}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::H , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
/// 	enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
///
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::Z , T::LAST);
/// }{
/// 	#[derive(Debug,PartialEq,EnumEnds)]
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
/// 	assert_eq!(T::A , T::FIRST);
/// 	assert_eq!(T::H , T::LAST);
/// }
/// ```
pub trait Ends: Sized{
	/// The first variant in a defined order of the enum.
	const FIRST: Self;

	/// The last variant in a defined order of the enum.
	const LAST: Self;
}

/// An enum item type that have a corresponding iterator iterating over all variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumIterable)]`][enum_traits_macros::EnumIterable].
pub trait Iterable{
	/// The type of the iterator
	type Iter: Iterator<Item = Self>;

	/// Constructs an iterator that iterates over every variant in the defined order
	fn variants() -> Self::Iter;
}

/// A function converting from a variant to its defined name.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumVariantName)]`][enum_traits_macros::EnumVariantName].
///
/// # Examples of correctness
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
pub trait VariantName{
	/// The name of the currently instantiated variant
	fn variant_name(&self) -> &'static str;
}

/// An enum item type that have a corresponding enum consisting of only unit variants describing the discriminants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumTag)]`][enum_traits_macros::EnumTag].
///
/// # Example of correctness
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
/// # Expanded example
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
///
/// // <rest is omitted>
/// ```
pub trait Tag{
	type Tag;

	/// The tag (unit variant) of the currently instantiated variant
	fn tag(&self) -> Self::Tag;

	/// The tag (unit variant) of the currently instantiated variant
	fn into_tag(self) -> Self::Tag;
}

/// The next and previous value of an enum based on an order on the variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumStep)]`][enum_traits_macros::EnumStep].
///
/// # Examples of correctness
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
/// {
/// 	#[derive(Debug,PartialEq,EnumStep)]
/// 	enum T{}
/// }{
/// 	#[derive(Debug,PartialEq,EnumStep)]
/// 	enum T{A}
///
/// 	assert_eq!(None , T::A.next());
///
/// 	assert_eq!(None , T::A.previous());
/// }{
/// 	#[derive(Debug,PartialEq,EnumStep)]
/// 	enum T{A,B}
///
/// 	assert_eq!(Some(T::B) , T::A.next());
/// 	assert_eq!(None       , T::B.next());
///
/// 	assert_eq!(None       , T::A.previous());
/// 	assert_eq!(Some(T::A) , T::B.previous());
/// }{
/// 	#[derive(Debug,PartialEq,EnumStep)]
/// 	enum T{A,B,C}
///
/// 	assert_eq!(Some(T::B) , T::A.next());
/// 	assert_eq!(Some(T::C) , T::B.next());
/// 	assert_eq!(None       , T::C.next());
///
/// 	assert_eq!(None       , T::A.previous());
/// 	assert_eq!(Some(T::A) , T::B.previous());
/// 	assert_eq!(Some(T::B) , T::C.previous());
/// }{
/// 	#[derive(Debug,PartialEq,EnumStep)]
/// 	enum T{A,B,C,D,E,F,G}
///
/// 	assert_eq!(Some(T::B) , T::A.next());
/// 	assert_eq!(Some(T::C) , T::B.next());
/// 	assert_eq!(Some(T::D) , T::C.next());
/// 	assert_eq!(Some(T::E) , T::D.next());
/// 	assert_eq!(Some(T::F) , T::E.next());
/// 	assert_eq!(Some(T::G) , T::F.next());
/// 	assert_eq!(None       , T::G.next());
///
/// 	assert_eq!(None       , T::A.previous());
/// 	assert_eq!(Some(T::A) , T::B.previous());
/// 	assert_eq!(Some(T::B) , T::C.previous());
/// 	assert_eq!(Some(T::C) , T::D.previous());
/// 	assert_eq!(Some(T::D) , T::E.previous());
/// 	assert_eq!(Some(T::E) , T::F.previous());
/// 	assert_eq!(Some(T::F) , T::G.previous());
/// }
/// ```
pub trait Step: Sized{
	/// The next variant in a defined order of the enum.
	fn next(self) -> Option<Self>;

	/// The previous variant in a defined order of the enum.
	fn previous(self) -> Option<Self>;
}

/// Represents an enum type that can be converted into a representation of its discriminant type.
///
/// This is primarily used by the automatic derivation [`#[derive(EnumFromDiscriminant)]`][enum_traits_macros::EnumFromDiscriminant].
///
/// # Example implementation
///
/// ```rust
/// use enum_traits::*;
///
/// #[repr(u8)]
/// pub enum Enum{A,B,C,D}
///
/// impl IntoDiscriminant<u8> for Enum{
/// 	#[inline(always)] fn into_discriminant(self) -> u8{self as u8}
/// }
/// ```
pub trait IntoDiscriminant<D>{
	/// Converts a value of the enum to its discriminant value.
	fn into_discriminant(self) -> D;
}

impl<T> IntoDiscriminant<mem::Discriminant<T>> for T{
	#[inline(always)]
	fn into_discriminant(self) -> mem::Discriminant<T>{
		mem::discriminant(&self)
	}
}

/// Implements [`IntoDiscriminant`] for a numeric type using an `as`-cast.
///
/// # Example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C}
///
/// impl_IntoDiscriminant_of_numeric!(u8,Enum);
/// ```
///
/// # Expanded example
///
/// ```rust
/// use enum_traits::*;
///
/// enum Enum{A,B,C}
///
/// impl IntoDiscriminant<u8> for Enum{
/// 	#[inline(always)]
/// 	fn into_discriminant(self) -> u8{
/// 		self as u8
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! impl_IntoDiscriminant_of_numeric{
	($num:ty,$ty:ty) => {
		impl ::enum_traits::IntoDiscriminant<$num> for $ty{
			#[inline(always)]
			fn into_discriminant(self) -> $num{
				self as $num
			}
		}
	};
}

/// Represents an enum type that can be converted from a representation of its discriminant type.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumFromDiscriminant)]`][enum_traits_macros::EnumFromDiscriminant].
///
/// # Examples of correctness
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromDiscriminant,Debug,PartialEq)]
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
/// assert_eq!(Some(Enum::A) , Enum::from_discriminant(8));
/// assert_eq!(Some(Enum::B) , Enum::from_discriminant(Enum::B as u8));
/// assert_eq!(Some(Enum::C) , Enum::from_discriminant(5));
/// assert_eq!(Some(Enum::D) , Enum::from_discriminant(7));
/// assert_eq!(None          , Enum::from_discriminant(0));
///
/// assert_eq!(Enum::A , unsafe{Enum::from_discriminant_unchecked(8)});
/// assert_eq!(Enum::B , unsafe{Enum::from_discriminant_unchecked(Enum::B as u8)});
/// assert_eq!(Enum::C , unsafe{Enum::from_discriminant_unchecked(5)});
/// assert_eq!(Enum::D , unsafe{Enum::from_discriminant_unchecked(7)});
/// ```
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumFromDiscriminant,Debug,PartialEq)]
/// pub enum Enum{
/// 	A,
/// 	B,
/// 	C,
/// 	D,
/// 	E,
/// }
///
/// impl IntoDiscriminant<u8> for Enum{
/// 	#[inline(always)] fn into_discriminant(self) -> u8{self as u8}
/// }
///
/// assert_eq!(Some(Enum::A) , Enum::from_discriminant(Enum::A as u8));
/// assert_eq!(Some(Enum::B) , Enum::from_discriminant(Enum::B as u8));
/// assert_eq!(Some(Enum::C) , Enum::from_discriminant(Enum::C as u8));
/// assert_eq!(Some(Enum::D) , Enum::from_discriminant(Enum::D as u8));
/// assert_eq!(Some(Enum::E) , Enum::from_discriminant(Enum::E as u8));
/// assert_eq!(None          , Enum::from_discriminant(100));
///
/// assert_eq!(Enum::A , unsafe{Enum::from_discriminant_unchecked(Enum::A as u8)});
/// assert_eq!(Enum::B , unsafe{Enum::from_discriminant_unchecked(Enum::B as u8)});
/// assert_eq!(Enum::C , unsafe{Enum::from_discriminant_unchecked(Enum::C as u8)});
/// assert_eq!(Enum::D , unsafe{Enum::from_discriminant_unchecked(Enum::D as u8)});
/// assert_eq!(Enum::E , unsafe{Enum::from_discriminant_unchecked(Enum::E as u8)});
/// ```
pub trait FromDiscriminant<D>: Sized{
	/// Constructs a possible value of the enum from a discriminant value, returning `None` if it is not a discriminant value of the enum.
	fn from_discriminant(d: D) -> Option<Self>;

	/// Constructs a value of the enum from its discriminant value (assuming it is an actual discriminant value).
	unsafe fn from_discriminant_unchecked(d: D) -> Self;
}

/*
/// Represents an enum type that have an array that lists all of the variants.
///
/// Derive this trait for an enum automatically using [`#[derive(EnumArray)]`][enum_traits_macros::EnumArray].
///
/// # Examples of correctness
///
/// ```rust
/// use enum_traits::*;
/// use enum_traits_macros::*;
///
/// #[derive(EnumVariantsArray,EnumLen,Debug,PartialEq)]
/// pub enum Enum{
/// 	A = 8,
/// 	B,
/// 	C = 5,
/// 	D = 7,
/// }
///
/// assert_eq!([Enum::A,Enum::B,Enum::C,Enum::D] , Enum::VARIANTS);
/// ```
pub trait VariantsArray: Sized + Len where [(); <Self as Len>::LEN]:{
	/// An array containing all the variants.
	const VARIANTS: [Self; <Self as Len>::LEN];
}
*/
