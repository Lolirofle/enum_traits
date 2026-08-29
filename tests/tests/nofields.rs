use enum_traits::*;
use enum_traits_macros::*;

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIs,EnumTag,EnumEnds,EnumIterator,EnumIterable,EnumVariantName,EnumFromVariantName)]
enum NoFields{
	A,B,C,D,E,F
}

#[test]
fn index(){
	let _ = NoFields::E.index() as <NoFields as EnumIndex>::Type;
}

#[test]
fn from_index(){
	assert_eq!(Some(NoFields::A),NoFields::from_index(0));
	assert_eq!(Some(NoFields::B),NoFields::from_index(1));
	assert_eq!(Some(NoFields::C),NoFields::from_index(2));
	assert_eq!(Some(NoFields::D),NoFields::from_index(3));
	assert_eq!(Some(NoFields::E),NoFields::from_index(4));
	assert_eq!(Some(NoFields::F),NoFields::from_index(5));
}

#[test]
fn to_index(){
	assert_eq!(0,NoFields::A.index());
	assert_eq!(1,NoFields::B.index());
	assert_eq!(2,NoFields::C.index());
	assert_eq!(3,NoFields::D.index());
	assert_eq!(4,NoFields::E.index());
	assert_eq!(5,NoFields::F.index());

	assert_eq!(0,NoFields::A.into_index());
	assert_eq!(1,NoFields::B.into_index());
	assert_eq!(2,NoFields::C.into_index());
	assert_eq!(3,NoFields::D.into_index());
	assert_eq!(4,NoFields::E.into_index());
	assert_eq!(5,NoFields::F.into_index());
}

#[test]
fn len(){
	assert_eq!(6,<NoFields as EnumLen>::LEN);
}

#[test]
fn variant_name(){
	assert_eq!(NoFields::A.variant_name(),"A");
	assert_eq!(NoFields::B.variant_name(),"B");
	assert_eq!(NoFields::C.variant_name(),"C");
	assert_eq!(NoFields::D.variant_name(),"D");
	assert_eq!(NoFields::E.variant_name(),"E");
	assert_eq!(NoFields::F.variant_name(),"F");
}

#[test]
fn from_str(){
	use core::str::FromStr;

	let mut v: Result<NoFields,()>;
	assert_eq!({v=NoFields::from_str("A"); v},Ok(NoFields::A));
	assert_eq!({v=NoFields::from_str("B"); v},Ok(NoFields::B));
	assert_eq!({v=NoFields::from_str("C"); v},Ok(NoFields::C));
	assert_eq!({v=NoFields::from_str("D"); v},Ok(NoFields::D));
	assert_eq!({v=NoFields::from_str("E"); v},Ok(NoFields::E));
	assert_eq!({v=NoFields::from_str("F"); v},Ok(NoFields::F));
}
