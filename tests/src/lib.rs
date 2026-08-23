//#![feature(min_adt_const_params,const_param_ty_trait)]
#![allow(dead_code)]
#![no_std]

//Useful commands for testing:
//  cargo rustc -- -Z unstable-options --pretty=expanded --test
//  cargo expand-macros
//  cargo expand --test main
//  cargo expand --ugly --all-features > [FILE]
//  cargo -v rustc --release -- --emit=llvm-ir

use enum_traits_macros::*;

/*
//TODO: Also write tests on this
#[allow(non_snake_case)]
#[derive(EnumFieldStructs)]
enum Fields<'a,X,Y>{
	VariantA(i8),
	VariantB(i32),
	VariantC(u8,u16,u32),
	VariantD{d: (u8,i32)},
	VariantE{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
	VariantF,
	VariantG(X),
	VariantH(&'a X),
	VariantI(&'a Y),
	VariantJ{x: X,y: &'a Y},
	VariantK{x: i8},
}

pub mod attrs{
	use enum_traits_macros::*;

	/// ```no_compile
	/// tests::attrs::T::LENGTH;
	/// ```
	#[impl_enum_len(#[cfg(all(test,not(test)))] pub LENGTH)]
	#[derive(Eq,PartialEq,Debug)]
	pub enum T{A,B,C,D}
}
*/

/* TODO: An idea. Remove later
enum Test{A(u8),B(u16),C(u32)}
#[derive(Eq,PartialEq)] enum TestTags{A,B,C}
struct A(u8);
struct B(u16);
struct C(u32);

impl core::marker::ConstParamTy_ for TestTags{}

trait TestFields<const T: TestTags>{
	type Out;
}
impl TestFields<TestTags::A>{type Out = A;}
impl TestFields<TestTags::B>{type Out = B;}
impl TestFields<TestTags::C>{type Out = C;}
*/

/*
#[derive(EnumIs)]
#[enum_is(#[deprecated] pub(crate) fn Enum2HasACustomName)]
enum Enum2{
	A,
	B
}
*/

/*
#[derive(EnumEnds)]
enum EnumE{A,B(u32)}
*/

/*
#[impl_enum_first(24)]
enum EnumE{A,B}
*/

/*
enum EnumE{A,B(u32)}

impl core::convert::TryFrom<EnumE> for u32{
	type Error = ();
	fn try_from(value: EnumE) -> Result<Self,Self::Error>{
		match value{
			EnumE::B(x) => Ok(x),
			_ => Err(()),
		}
	}
}

impl<'l> core::convert::TryFrom<&'l EnumE> for &'l u32{
	type Error = ();
	fn try_from(value: &'l EnumE) -> Result<Self,Self::Error>{
		match value{
			EnumE::B(x) => Ok(x),
			_ => Err(()),
		}
	}
}
*/

/*
#[derive(EnumFrom)]
enum Enum1{
	A,
	B
}
#[derive(EnumFrom)]
enum Enum2{
	#[enum_from(disable)] A,
	B
}
*/

/*
#[derive(EnumIs)]
enum Enum{
	#[enum_is(exclude)]
	A,
	#[enum_is(name(pub custom_name))]
	B(u32),
	C{i: u32},
}
*/
