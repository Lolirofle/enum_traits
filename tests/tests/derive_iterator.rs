#![allow(dead_code)]
use core::iter::Iterator;
use enum_traits::*;
use enum_traits_macros::*;

#[test] fn one(){
	#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A}
	let mut t = T::FIRST;
	assert_eq!(T::A,t);        assert_eq!(t.len(),0);
	assert_eq!(None,t.next()); assert_eq!(t.len(),0);

	assert_eq!(T::LAST,T::A);
}

#[test] fn three(){
	#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C}
	let mut t = T::FIRST;

	assert_eq!(T::A,t);              assert_eq!(t.len(),2);
	assert_eq!(Some(T::B),t.next()); assert_eq!(t.len(),1);
	assert_eq!(Some(T::C),t.next()); assert_eq!(t.len(),0);
	assert_eq!(None      ,t.next()); assert_eq!(t.len(),0);

	assert_eq!(t.count(),0);
}

#[test] fn seven(){
	#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G}
	let mut t = T::FIRST;
	assert_eq!(T::A,t);
	assert_eq!(Some(T::B),t.next());
	assert_eq!(Some(T::C),t.next());
	assert_eq!(Some(T::D),t.next());
	assert_eq!(Some(T::E),t.next());
	assert_eq!(Some(T::F),t.next());
	assert_eq!(Some(T::G),t.next());
	assert_eq!(None      ,t.next());
}

#[test] fn eight(){
	#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H}
	let mut t = T::FIRST;
	assert_eq!(T::A,t);
	assert_eq!(Some(T::B),t.next());
	assert_eq!(Some(T::C),t.next());
	assert_eq!(Some(T::D),t.next());
	assert_eq!(Some(T::E),t.next());
	assert_eq!(Some(T::F),t.next());
	assert_eq!(Some(T::G),t.next());
	assert_eq!(Some(T::H),t.next());
	assert_eq!(None      ,t.next());
}

#[test] fn twentyfive(){
	#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
	let mut t = T::FIRST;
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
