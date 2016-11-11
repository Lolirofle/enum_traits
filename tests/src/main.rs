//cargo rustc -- -Z unstable-options --pretty=expanded --test

#![feature(associated_consts,proc_macro)]

#[macro_use]extern crate enum_traits_macros;
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

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter)]
enum NoFields{
	A,B,C,D,E,F
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter,EnumDiscriminant)]
enum Discriminants{
	A=1,B=2,C=4,D=8,E=16,F=33
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter,EnumDiscriminant)]
#[repr(u32)]
enum SomeDiscriminants{
	A=1,B,C=4,D,E=16,F
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

	let _ = e.index() as <Fields<i32> as Index>::Type;
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

#[test]
fn test_nofields_from_index(){
	assert_eq!(NoFields::A,NoFields::from_index(0).unwrap());
	assert_eq!(NoFields::B,NoFields::from_index(1).unwrap());
	assert_eq!(NoFields::C,NoFields::from_index(2).unwrap());
	assert_eq!(NoFields::D,NoFields::from_index(3).unwrap());
	assert_eq!(NoFields::E,NoFields::from_index(4).unwrap());
	assert_eq!(NoFields::F,NoFields::from_index(5).unwrap());
}

#[test]
fn test_nofields_index(){
	let _ = NoFields::E.index() as <Fields<i32> as Index>::Type;
}

#[test]
fn test_discriminants(){
	assert_eq!(None                  ,Discriminants::from_discriminant(0));
	assert_eq!(Some(Discriminants::A),Discriminants::from_discriminant(1));
	assert_eq!(Some(Discriminants::B),Discriminants::from_discriminant(2));
	assert_eq!(None                  ,Discriminants::from_discriminant(3));
	assert_eq!(Some(Discriminants::C),Discriminants::from_discriminant(4));
	assert_eq!(Some(Discriminants::D),Discriminants::from_discriminant(8));
	assert_eq!(Some(Discriminants::E),Discriminants::from_discriminant(16));
	assert_eq!(None                  ,Discriminants::from_discriminant(32));
	assert_eq!(Some(Discriminants::F),Discriminants::from_discriminant(33));
}


#[allow(dead_code)]
#[test]
fn test_len(){
	assert_eq!(6,Fields::<'static,u32>::LEN);
	assert_eq!(6,NoFields::LEN);
	assert_eq!(6,Discriminants::LEN);

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


#[allow(dead_code)]
#[test]
fn test_iterator(){
	use std::iter::Iterator;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(None,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(Some(T::I),t.next());
		assert_eq!(Some(T::J),t.next());
		assert_eq!(Some(T::K),t.next());
		assert_eq!(Some(T::L),t.next());
		assert_eq!(Some(T::M),t.next());
		assert_eq!(Some(T::N),t.next());
		assert_eq!(Some(T::O),t.next());
		assert_eq!(Some(T::P),t.next());
		assert_eq!(Some(T::Q),t.next());
		assert_eq!(Some(T::R),t.next());
		assert_eq!(Some(T::S),t.next());
		assert_eq!(Some(T::T),t.next());
		assert_eq!(Some(T::U),t.next());
		assert_eq!(Some(T::V),t.next());
		assert_eq!(Some(T::X),t.next());
		assert_eq!(Some(T::Y),t.next());
		assert_eq!(Some(T::Z),t.next());
		assert_eq!(None      ,t.next());
	}
}

#[allow(dead_code)]
#[test]
fn test_iter(){
	use std::iter::Iterator;
	use enum_traits::Iterable;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(None,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G,H}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(Some(T::I),t.next());
		assert_eq!(Some(T::J),t.next());
		assert_eq!(Some(T::K),t.next());
		assert_eq!(Some(T::L),t.next());
		assert_eq!(Some(T::M),t.next());
		assert_eq!(Some(T::N),t.next());
		assert_eq!(Some(T::O),t.next());
		assert_eq!(Some(T::P),t.next());
		assert_eq!(Some(T::Q),t.next());
		assert_eq!(Some(T::R),t.next());
		assert_eq!(Some(T::S),t.next());
		assert_eq!(Some(T::T),t.next());
		assert_eq!(Some(T::U),t.next());
		assert_eq!(Some(T::V),t.next());
		assert_eq!(Some(T::X),t.next());
		assert_eq!(Some(T::Y),t.next());
		assert_eq!(Some(T::Z),t.next());
		assert_eq!(None      ,t.next());
	}
}

