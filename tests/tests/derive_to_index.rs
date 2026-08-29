use enum_traits::*;
use enum_traits_macros::*;

#[test] fn empty(){
	#[allow(unused)] #[derive(EnumIndex,EnumToIndex)] enum X{}
}

#[test] fn single(){
	#[derive(EnumIndex,EnumToIndex)] enum X{A}
	assert_eq!(X::A.into_index(),0);
}

#[test] fn many(){
	#[derive(EnumIndex,EnumToIndex)] enum X{A,B,C,D,E,F}
	assert_eq!(X::A.into_index(),0);
	assert_eq!(X::B.into_index(),1);
	assert_eq!(X::C.into_index(),2);
	assert_eq!(X::D.into_index(),3);
	assert_eq!(X::E.into_index(),4);
	assert_eq!(X::F.into_index(),5);
}
