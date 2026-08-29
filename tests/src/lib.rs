#![allow(dead_code)]
#![no_std]

mod attr_ends;
mod attr_field_structs;
mod attr_len;
mod derive_from;
mod derive_into;

#[macro_export] macro_rules! gen_test_attr_const{
	($e: expr,$name: tt ($($pre: tt)?) ($($post: tt)?) $(, #[ $($derives: tt)* ])?) => {
		#[test] fn no_prefix(){
			$(#[ $($derives)* ])? #[$name($($pre)* NAME $($post)*)] enum X{A,B,C}
			assert_eq!(X::NAME,$e);
		}

		#[test] fn const_prefix(){
			$(#[ $($derives)* ])? #[$name($($pre)* const NAME $($post)*)] enum X{A,B,C}
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_crate_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(crate) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_vis_const(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub const NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_super_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(super) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_in_path_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(in super) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn attr_conditional(){
			$(#[ $($derives)* ])? #[$name(#[cfg(all(test,not(test)))] $($pre)* NAME $($post)*)] enum X{A,B,C}
			impl X{const NAME: u64 = 0x85ce938eaf004a38;}
			assert_eq!(X::NAME,0x85ce938eaf004a38);
		}
	};
}

//Useful commands for testing:
//  cargo rustc -- -Z unstable-options --pretty=expanded --test
//  cargo expand-macros
//  cargo expand --test main
//  cargo expand --ugly --all-features > [FILE]
//  cargo -v rustc --release -- --emit=llvm-ir

use enum_traits_macros::*;

/*#[derive(EnumInto)]
enum Enum{
	A(u8,u16,u32),
	B(i32,u16,u8),
	C{x: i32,y: u8,z: u16,w: &'static str},
	D{o: u8,p: u16,q: u8},
}*/

/*
#[derive(EnumInto)]
enum Enum2{
	A(u8,u16),
	B(i32,i64),
	D{o: u32,p: u64},
}
*/

/*
#[derive(EnumInto)]
enum Enum3{
	A(u8,u16),
	B(&'static str,u8),
	C{i: u8,b: bool},
}
*/

/*
#[derive(EnumInto)]
enum Enum3{
	A(u8,u16,u32,i64),
	B{x: u32,y: i64},
	C{z: i64,y: i64,w: u32},
	D{x: i64,y: i64,z: i64,a: u32,b: u32},
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
