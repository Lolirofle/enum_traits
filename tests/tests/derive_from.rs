#![allow(dead_code)]
use enum_traits_macros::*;
use core::fmt::Debug;

#[test] fn empty(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum E{}
}

#[test] fn single_unit_variant(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum E{
		Unit,
	}
	assert_eq!(E::from(()),E::Unit);
}


#[test] fn single_tuple_variant(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum E{
		T(u8),
	}
	assert_eq!(E::from(3u8),E::T(3));
}

#[test] fn order_dependent(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum E{
		X{a: u8,b: u16},
		Y{p: u16,q: u8},
	}
	assert_eq!(E::from((1u8,2u16)),E::X{a: 1,b: 2});
	assert_eq!(E::from((2u16,1u8)),E::Y{p: 2,q: 1});
}


#[test] fn generic_two_field_tuple(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum E<A,B>{
		P(A,B),
	}
	assert_eq!(E::<i32,&str>::from((1,"hi")),E::P(1,"hi"));
	assert_eq!(E::<u8,u16>::from((1u8,2u16)),E::P(1,2));
}

#[test] fn generic_two_field_struct(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum PairStruct<A,B>{
		S{a: A,b: B},
	}
	assert_eq!(
		PairStruct::<i32,u8>::from((1,2u8)),
		PairStruct::S{a: 1,b: 2},
	);
}

#[test] fn where_bounds(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum Bounded<T> where
		T: Clone + Debug,
	{
		V(T,T),
	}
	assert_eq!(Bounded::from(("a","b")),Bounded::V("a","b"));
	assert_eq!(Bounded::from((1u8,2u8)),Bounded::V(1,2));
}

#[test] fn lifetime(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum StrOrBytes<'a>{
		S(&'a str),
		B(&'a [u8]),
	}
	let s: &str = "hello";
	let b: &[u8] = b"world";

	assert_eq!(StrOrBytes::from(s),StrOrBytes::S("hello"));
	assert_eq!(StrOrBytes::from(b),StrOrBytes::B(b"world"));
}

#[test] fn lifetime_and_generic(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum RefPair<'a,T>{
		R(&'a T),
		V(T,T),
	}
	let x = 5i32;
	assert_eq!(RefPair::from(&x),RefPair::R(&5));
	assert_eq!(RefPair::from((1u8,2u8)),RefPair::V(1,2));
}

#[test] fn const_generics(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum ArrWrap<const N: usize>{
		A([u8; N]),
	}
	assert_eq!(ArrWrap::<3>::from([1u8,2,3]),ArrWrap::A([1,2,3]));
	assert_eq!(ArrWrap::<0>::from([]),ArrWrap::A([]));
}

#[test] fn wrapper(){
	#[derive(EnumFrom,Debug,PartialEq,Eq)]
	enum Wrap<T>{
		V(T),
	}
	assert_eq!(Wrap::from(5u8),Wrap::V(5u8));
	assert_eq!(Wrap::from("abc"),Wrap::V("abc"));
}

#[test] fn exclude(){
	#[derive(EnumFrom)]
	enum E{
		A,
		#[enum_from(exclude)] B,
	}
}
