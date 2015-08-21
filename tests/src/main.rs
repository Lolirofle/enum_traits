#![feature(plugin,custom_derive,custom_attribute)]
#![plugin(enum_traits_macros)]

extern crate enum_traits;

use enum_traits::*;

#[derive(Copy,Clone,EnumIndex,EnumToIndex,EnumLen)]
enum Fields<'t,T: 't>{
	VariantA(&'t T),
	VariantB(T),
	VariantC(T,T,T,T),
	VariantD{d: i32},
	VariantE{a: i32,b: i32,c: i32,d: i32,e: i32},
	VariantF,
}


#[derive(EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator)]
enum NoFields{
	A,B,C,D,E,F
}

#[test]
fn test_fields_index(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert_eq!(0,e.index());

	e = Fields::VariantB(0);
	assert_eq!(1,e.index());

	e = Fields::VariantC(0,1,2,3);
	assert_eq!(2,e.index());

	e = Fields::VariantD{d: 0};
	assert_eq!(3,e.index());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert_eq!(4,e.index());

	e = Fields::VariantF;
	assert_eq!(5,e.index());
}

#[test]
fn test_fields_into_index(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert_eq!(0,e.into_index());

	e = Fields::VariantB(0);
	assert_eq!(1,e.into_index());

	e = Fields::VariantC(0,1,2,3);
	assert_eq!(2,e.into_index());

	e = Fields::VariantD{d: 0};
	assert_eq!(3,e.into_index());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert_eq!(4,e.into_index());

	e = Fields::VariantF;
	assert_eq!(5,e.into_index());
}

#[allow(dead_code)]
#[test]
fn test_len(){
	assert_eq!(6,Fields::<'static,u32>::LEN);

	{
		#[derive(EnumLen)]enum T{}
		assert_eq!(0,T::LEN);
	}

	{
		#[derive(EnumLen)]enum T{A}
		assert_eq!(1,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C}
		assert_eq!(3,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G}
		assert_eq!(7,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H}
		assert_eq!(8,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(25,T::LEN);
	}

	{
		#[derive(EnumLen)]enum T{A(u32)}
		assert_eq!(1,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64}}
		assert_eq!(3,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G}
		assert_eq!(7,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G,H}
		assert_eq!(8,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(25,T::LEN);
	}
}

#[allow(dead_code)]
#[test]
fn test_ends(){
	{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A}
		assert_eq!(T::A,T::first());
		assert_eq!(T::A,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C}
		assert_eq!(T::A,T::first());
		assert_eq!(T::C,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G}
		assert_eq!(T::A,T::first());
		assert_eq!(T::G,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H}
		assert_eq!(T::A,T::first());
		assert_eq!(T::H,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(T::A,T::first());
		assert_eq!(T::Z,T::last());
	}
}

/*
#[allow(dead_code)]
#[test]
fn test_iter(){
	use std::iter::Iterator;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator,EnumIndex,EnumFromIndex,EnumToIndex)]enum T{A}
		let mut t = T::first();
		//assert_eq!(T::A,t.next().unwrap());
		//assert!(t.next().is_none());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator,EnumIndex,EnumFromIndex,EnumToIndex)]enum T{A,B,C}
		let mut t = T::first();
		assert_eq!(T::A,t.next().unwrap());
		assert_eq!(T::B,t.next().unwrap());
		assert_eq!(T::C,t.next().unwrap());
		assert!(t.next().is_none());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator,EnumIndex,EnumFromIndex,EnumToIndex)]enum T{A,B,C,D,E,F,G}
		let mut t = T::first();
		assert_eq!(T::A,t.next().unwrap());
		assert_eq!(T::B,t.next().unwrap());
		assert_eq!(T::C,t.next().unwrap());
		assert_eq!(T::D,t.next().unwrap());
		assert_eq!(T::E,t.next().unwrap());
		assert_eq!(T::F,t.next().unwrap());
		assert_eq!(T::G,t.next().unwrap());
		assert!(t.next().is_none());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator,EnumIndex,EnumFromIndex,EnumToIndex)]enum T{A,B,C,D,E,F,G,H}
		let mut t = T::first();
		assert_eq!(T::A,t.next().unwrap());
		assert_eq!(T::B,t.next().unwrap());
		assert_eq!(T::C,t.next().unwrap());
		assert_eq!(T::D,t.next().unwrap());
		assert_eq!(T::E,t.next().unwrap());
		assert_eq!(T::F,t.next().unwrap());
		assert_eq!(T::G,t.next().unwrap());
		assert_eq!(T::H,t.next().unwrap());
		assert!(t.next().is_none());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator,EnumIndex,EnumFromIndex,EnumToIndex)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		let mut t = T::first();
		assert_eq!(T::A,t.next().unwrap());
		assert_eq!(T::B,t.next().unwrap());
		assert_eq!(T::C,t.next().unwrap());
		assert_eq!(T::D,t.next().unwrap());
		assert_eq!(T::E,t.next().unwrap());
		assert_eq!(T::F,t.next().unwrap());
		assert_eq!(T::G,t.next().unwrap());
		assert_eq!(T::H,t.next().unwrap());
		assert_eq!(T::I,t.next().unwrap());
		assert_eq!(T::J,t.next().unwrap());
		assert_eq!(T::K,t.next().unwrap());
		assert_eq!(T::L,t.next().unwrap());
		assert_eq!(T::M,t.next().unwrap());
		assert_eq!(T::N,t.next().unwrap());
		assert_eq!(T::O,t.next().unwrap());
		assert_eq!(T::P,t.next().unwrap());
		assert_eq!(T::Q,t.next().unwrap());
		assert_eq!(T::R,t.next().unwrap());
		assert_eq!(T::S,t.next().unwrap());
		assert_eq!(T::T,t.next().unwrap());
		assert_eq!(T::U,t.next().unwrap());
		assert_eq!(T::V,t.next().unwrap());
		assert_eq!(T::X,t.next().unwrap());
		assert_eq!(T::Y,t.next().unwrap());
		assert_eq!(T::Z,t.next().unwrap());
		assert!(t.next().is_none());
	}
}
*/
