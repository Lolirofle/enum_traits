#![allow(non_camel_case_types)]

use enum_traits_macros::*;
use std::convert::TryFrom;

#[test]
fn single(){
	#[derive(EnumTryInto)]
	enum E{B(u8,u16)}

	assert_eq!(u8::try_from(E::B(1,2)).unwrap(),1);
	assert_eq!(u16::try_from(E::B(1,2)).unwrap(),2);
}

#[test]
fn single2(){
	#[derive(EnumTryInto)]
	enum E{B(u8,u16)}

	assert_eq!(1u8,E::B(1,2).try_into().unwrap());
	assert_eq!(2u16,E::B(1,2).try_into().unwrap());
}

#[test]
fn units(){
	#[derive(EnumTryInto)]
	enum E{A,B,C}
}

#[test]
fn single_variant_single_field(){
	#[derive(EnumTryInto)]
	enum E{Only(u8)}

	assert_eq!(u8::try_from(E::Only(7)),Ok(7));
}

#[test]
fn simple(){
	#[derive(EnumTryInto)]
	enum E{
		A,
		B(u8,u16),
		C(u8),
	}

	assert!(u8::try_from(E::A).is_err());
	assert_eq!(u8::try_from(E::B(1,2)),Ok(1));
	assert_eq!(u8::try_from(E::C(9)),Ok(9));

	assert!(u16::try_from(E::A).is_err());
	assert_eq!(u16::try_from(E::B(3,4)),Ok(4));
	assert!(u16::try_from(E::C(9)).is_err());
}

#[test]
fn mix(){
	#[derive(EnumTryInto)]
	enum E{
		A(u8,u16),
		B{x: u16,y: u32},
	}

	assert_eq!(u8::try_from(E::A(1,2)),Ok(1));
	assert!(u8::try_from(E::B{x: 0,y: 0}).is_err());

	assert_eq!(u16::try_from(E::A(1,2)),Ok(2));
	assert_eq!(u16::try_from(E::B{x: 3,y: 0}),Ok(3));

	assert!(u32::try_from(E::A(5,6)).is_err());
	assert_eq!(u32::try_from(E::B{x: 0,y: 4}),Ok(4));
}

#[test]
fn duplicate(){
	#[derive(EnumTryInto)]
	enum E{
		A(u8),
		B(u8,u8),
		C{x: u8,y: u8},
	}

	assert_eq!(u8::try_from(E::A(1)),Ok(1));
	assert_eq!(u8::try_from(E::B(2,3)),Ok(2));
	assert_eq!(u8::try_from(E::C{x: 4,y: 5}),Ok(4));
}

#[test]
fn reference_field(){
	#[derive(EnumTryInto)]
	enum E<'a>{
		A(&'a u8),
		B(&'a str),
	}

	let x = 7u8;
	assert_eq!(<&u8>::try_from(E::A(&x)).unwrap(),&7);
	assert_eq!(<&str>::try_from(E::B("hi")).unwrap(),"hi");
}

#[test]
fn tuple_field(){
	#[derive(EnumTryInto)]
	enum E{
		A((u8,u16)),
	}

	assert_eq!(<(u8,u16)>::try_from(E::A((1,2))),Ok((1,2)));
}

#[test]
fn raw_variant_identifier(){
	#[derive(EnumTryInto)]
	enum E{
		r#type(u8),
	}

	assert_eq!(u8::try_from(E::r#type(7)),Ok(7));
}
