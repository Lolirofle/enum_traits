#![allow(dead_code)]
use enum_traits_macros::*;
use enum_traits_tests::*;

#[test] fn empty(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_variants_array(V)] enum X{}
	assert_eq!(X::V,[]);
}

#[test] fn single(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_variants_array(VA)] enum X{A}
	assert_eq!(X::VA,[X::A]);
}

#[test] fn many(){
	#[derive(Debug,Eq,PartialEq)] #[impl_enum_variants_array(VAR)] enum X{A,B,C,D,E,F}
	assert_eq!(X::VAR,[X::A,X::B,X::C,X::D,X::E,X::F]);
}

gen_test_attr_const!([X::A,X::B,X::C],impl_enum_variants_array()(),#[derive(Debug,Eq,PartialEq)]);
