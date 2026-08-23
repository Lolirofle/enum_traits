#![allow(dead_code)]
use enum_traits_macros::*;

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

#[test] fn pub_vis(){
	#[impl_enum_len(pub LENGTH)] enum X{A,B,C}
	assert_eq!(X::LENGTH,3);
}

#[test] fn pub_crate_vis(){
	#[impl_enum_len(pub(crate) LEN)] enum X{A,B}
	assert_eq!(X::LEN,2);
}

#[test] fn pub_vis_const(){
	#[impl_enum_len(pub const LEN)] enum X{A,B}
	assert_eq!(X::LEN,2);
}

#[test] fn pub_super_vis(){
	mod inner{use super::*; #[impl_enum_len(pub(super) LENG)] pub enum X{A}}
	assert_eq!(inner::X::LENG,1);
}

#[test] fn pub_in_path_vis(){
	mod inner{use super::*; #[impl_enum_len(pub(in super) LENGT)] pub enum X{A,B,C,D}}
	assert_eq!(inner::X::LENGT,4);
}

#[test] fn with_discriminants(){
	#[impl_enum_len(LE)] enum X{A = 10, B = 20, C = 30}
	assert_eq!(X::LE,3);
}

#[test] fn with_fields(){
	#[impl_enum_len(L)] enum X{A(u32),B{x: u8},C}
	assert_eq!(X::L,3);
}

