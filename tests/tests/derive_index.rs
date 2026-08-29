use enum_traits::*;
use enum_traits_macros::*;

#[test]
fn unit_variants(){
	#[derive(Debug,Eq,PartialEq,EnumFromIndex,EnumToIndex,EnumIndex)]
	enum T{
		A = 5,
		B = 1,
		C = 3,
		D,
	}

	assert_eq!(Some(T::A),T::from_index(0u8));
	assert_eq!(Some(T::B),T::from_index(1u8));
	assert_eq!(Some(T::C),T::from_index(2u8));
	assert_eq!(Some(T::D),T::from_index(3u8));
	assert_eq!(None      ,T::from_index(4u8));

	assert_eq!(0u8,T::A.index());
	assert_eq!(1u8,T::B.index());
	assert_eq!(2u8,T::C.index());
	assert_eq!(3u8,T::D.index());
}

#[test]
fn any_variants(){
	#[derive(Debug,Eq,PartialEq,EnumToIndex,EnumIndex)]
	#[repr(u8)]
	enum T{
		A = 5,
		B,
		C(u16),
		D{x: i8 , y: i64},
	}

	assert_eq!(0u8,T::A.index());
	assert_eq!(1u8,T::B.index());
	assert_eq!(2u8,T::C(16).index());
	assert_eq!(3u8,T::D{x: 32 , y: 64}.index());
}
