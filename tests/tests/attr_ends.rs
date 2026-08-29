#![allow(dead_code)]
use enum_traits_macros::*;

#[test] fn one(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::A,T::LAS);
}

#[test] fn three(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A,B,C}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::C,T::LAS);
}

#[test] fn seven(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A,B,C,D,E,F,G}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::G,T::LAS);
}

#[test] fn eight(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A,B,C,D,E,F,G,H}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::H,T::LAS);
}

#[test] fn twentyfive(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::Z,T::LAS);
}

#[test] fn mix(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] #[impl_enum_last(LAS)] enum T{A,B(u64),C{c: i32},D}
	assert_eq!(T::A,T::FIR);
	assert_eq!(T::D,T::LAS);
}

#[test] fn mix_first(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_first(FIR)] enum T{A,B(u64),C{c: i32}}
	assert_eq!(T::A,T::FIR);
}

#[test] fn mix_last(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_last(LAS)] enum T{B(u64),C{c: i32},D}
	assert_eq!(T::D,T::LAS);
}

mod first_vis{
	use enum_traits_macros::*;
	use enum_traits_tests::*;
	gen_test_attr_const!(X::A,impl_enum_first()(),#[derive(Debug,Eq,PartialEq)]);
}

mod last_vis{
	use enum_traits_macros::*;
	use enum_traits_tests::*;
	gen_test_attr_const!(X::C,impl_enum_last()(),#[derive(Debug,Eq,PartialEq)]);
}
