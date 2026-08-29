#![allow(dead_code)]
use enum_traits_macros::*;
use enum_traits_tests::*;

#[test] fn name_empty(){
	#[impl_enum_len(LENGTH)] enum X{}
	assert_eq!(X::LENGTH,0);
}

#[test] fn name_single(){
	#[impl_enum_len(LENGTH)] enum X{A}
	assert_eq!(X::LENGTH,1);
}

#[test] fn name_many(){
	#[impl_enum_len(COUNT)] enum X{A,B,C,D,E,F}
	assert_eq!(X::COUNT,6);
}

#[test] fn with_discriminants(){
	#[impl_enum_len(LE)] enum X{A = 10, B = 20, C = 30, D = 40, E = 50}
	assert_eq!(X::LE,5);
}

#[test] fn with_fields(){
	#[impl_enum_len(L)] enum X{A(u32),B{x: u8},C,D}
	assert_eq!(X::L,4);
}

gen_test_attr_const!(3,impl_enum_len()());
