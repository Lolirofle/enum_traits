use enum_traits::*;
use enum_traits_macros::*;

#[test]
fn explicit(){
	#[derive(Debug,Eq,PartialEq,EnumFromDiscriminant)]
	enum T{
		A = 5,
		B = 1,
		C = 3,
		D = 7,
	}

	impl_IntoDiscriminant_of_numeric!(u8,T);

	assert_eq!(Some(T::A),T::from_discriminant(5u8));
	assert_eq!(Some(T::B),T::from_discriminant(1u8));
	assert_eq!(Some(T::C),T::from_discriminant(3u8));
	assert_eq!(Some(T::D),T::from_discriminant(7u8));
	assert_eq!(None      ,T::from_discriminant(2u8));
}

#[test]
fn implicit(){ //See https://doc.rust-lang.org/nightly/reference/items/enumerations.html#implicit-discriminants
	#[derive(Debug,Eq,PartialEq,EnumFromDiscriminant)]
	enum T{
		A,
		B,
		C,
		D,
	}

	impl_IntoDiscriminant_of_numeric!(u16,T);

	assert_eq!(Some(T::A),T::from_discriminant(0u16));
	assert_eq!(Some(T::B),T::from_discriminant(1u16));
	assert_eq!(Some(T::C),T::from_discriminant(2u16));
	assert_eq!(Some(T::D),T::from_discriminant(3u16));
	assert_eq!(None      ,T::from_discriminant(4u16));
}

#[test]
fn builtin(){
	use core::mem::discriminant;

	#[derive(Debug,Eq,PartialEq,EnumFromDiscriminant)]
	enum T{
		A,
		B,
		C,
		D,
	}

	assert_eq!(Some(T::A),T::from_discriminant(discriminant(&T::A)));
	assert_eq!(Some(T::B),T::from_discriminant(discriminant(&T::B)));
	assert_eq!(Some(T::C),T::from_discriminant(discriminant(&T::C)));
	assert_eq!(Some(T::D),T::from_discriminant(discriminant(&T::D)));
}
