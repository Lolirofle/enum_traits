use enum_traits::*;
use enum_traits_macros::*;

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIterable)]
enum Discriminants{
	A=1,B=2,C=4,D=8,E=16,F=33
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIterable)]
#[repr(u32)]
enum SomeDiscriminants{
	A=1,B,C=4,D,E=16,F
}

#[test]
fn len(){
	assert_eq!(6,<Discriminants as Len>::LEN);
	assert_eq!(6,<SomeDiscriminants as Len>::LEN);
}
