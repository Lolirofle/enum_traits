#![allow(dead_code)]
use enum_traits::*;
use enum_traits_macros::*;

#[test] fn one(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::A,T::LAST);
}

#[test] fn three(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::C,T::LAST);
}

#[test] fn seven(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::G,T::LAST);
}

#[test] fn eight(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::H,T::LAST);
}

#[test] fn twentyfive(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::Z,T::LAST);
}

#[test] fn mix(){
	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B(u64),C{c: i32},D}
	assert_eq!(T::A,T::FIRST);
	assert_eq!(T::D,T::LAST);
}
