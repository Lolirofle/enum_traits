#![allow(dead_code)]
#![no_std]

mod attr_ends;
mod attr_len;
mod derive_from;

//Useful commands for testing:
//  cargo rustc -- -Z unstable-options --pretty=expanded --test
//  cargo expand-macros
//  cargo expand --test main
//  cargo expand --ugly --all-features > [FILE]
//  cargo -v rustc --release -- --emit=llvm-ir

use enum_traits_macros::*;

#[derive(EnumInto)]
enum Enum{
	A(u8,u16,u32),
	B(i32,u16,u8),
	C{x: i32,y: u8,z: u16,w: &'static str},
	D{o: u8,p: u16,q: u8},
}

/*
#[derive(EnumInto)]
enum Enum2{
	A(u8,u16),
	B(i32,i64),
	D{o: u32,p: u64},
}
*/

#[derive(EnumInto)]
enum Enum3{
	A(u8,u16,u32,i64),
	B{x: u32,y: i64},
	C{z: i64,y: i64,w: u32},
	D{x: i64,y: i64,z: i64,a: u32,b: u32},
}

/*#[derive(EnumFieldStructs)]
enum Fields<'a,X,Y>{
	A(i8),
	B(i32),
	C(u8,u16,u32),
	D{d: (u8,i32)},
	E{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
	F,
	G(X),
	H(&'a Y),
	I{x: X,y: &'a Y},
	J{x: i8},
}*/

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
*/

/*
#[derive(EnumIs)]
enum Enum2{
	A,
	#[enum_is(name(#[deprecated] pub(crate) fn))] B
}
*/

/*TODO: Implement something like this. And it would probably be more useful if the type is searched for in each field. But what happens for duplicate types in fields? Probably just error
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
#[derive(EnumIs)]
enum Enum{
	#[enum_is(exclude)]
	A,
	#[enum_is(name(pub custom_name))]
	B(u32),
	C{i: u32},
}
*/

