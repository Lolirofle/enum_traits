use enum_traits_macros::*;

#[test] fn empty(){
	#[allow(unused)] #[impl_enum_to_index(fn i(self) -> u8)] enum X{}
}

#[test] fn single(){
	#[impl_enum_to_index(fn i(self) -> u16)] enum X{A}
	assert_eq!(X::A.i(),0);
}

#[test] fn many(){
	#[impl_enum_to_index(fn i(&self) -> i32)] enum X{A,B,C,D,E,F}
	assert_eq!(X::A.i(),0);
	assert_eq!(X::B.i(),1);
	assert_eq!(X::C.i(),2);
	assert_eq!(X::D.i(),3);
	assert_eq!(X::E.i(),4);
	assert_eq!(X::F.i(),5);
}
