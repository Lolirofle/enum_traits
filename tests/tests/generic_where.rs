#![allow(dead_code)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use enum_traits_macros::*;

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumToIndex,EnumLen,EnumIs,EnumTag,EnumVariantName,EnumFromVariantName)]
enum Generic<'x,'y: 'x,'z,X,Y: 'y,Z> where
	'z: 'y,
	X: 'x,
	Y: Iterator,
	<Y as Iterator>::Item: core::fmt::Debug + Clone + Eq,
	u32: From<<Y as Iterator>::Item>,
{
	A,
	B(&'x X),
	C(Box<X>,Vec<Y>),
	D{y: Vec<Y>,z: &'z Z},
	E{x: Vec<X>,y: &'y Y,z: Box<Z>},
	F{i: i32,z: &'z Z},
	G(u32,i32,u8),
	H{i: i32,u: u32},
	I(<Y as Iterator>::Item),
}
