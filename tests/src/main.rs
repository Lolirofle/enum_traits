//cargo rustc -- -Z unstable-options --pretty=expanded --test

#![feature(associated_consts)]
#![allow(unreachable_code)]

#[macro_use]extern crate enum_traits_macros;
extern crate enum_traits;

use enum_traits::*;

#[derive(Copy,Clone,Debug,Eq,PartialEq,EnumIndex,EnumToIndex,EnumLen,EnumDiscriminant,EnumIsVariantFns,EnumTag,EnumVariantName,EnumFromVariantName)]
enum Fields<'t,T: 't>{
	VariantA(&'t T),
	VariantB(T),
	VariantC(T,T,T,T),
	VariantD{d: i32},
	VariantE{a: i32,b: i32,c: i32,d: i32,e: i32},
	VariantF,
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter,EnumDiscriminant,EnumFromVariantName)]
enum NoFields{
	A,B,C,D,E,F
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter,EnumDiscriminant)]
enum Discriminants{
	A=1,B=2,C=4,D=8,E=16,F=33
}

#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator,EnumIter,EnumDiscriminant)]
#[repr(u32)]
enum SomeDiscriminants{
	A=1,B,C=4,D,E=16,F
}

#[test]
fn test_fields_index(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert_eq!(0,e.index());

	e = Fields::VariantB(0);
	assert_eq!(1,e.index());

	e = Fields::VariantC(0,1,2,3);
	assert_eq!(2,e.index());

	e = Fields::VariantD{d: 0};
	assert_eq!(3,e.index());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert_eq!(4,e.index());

	e = Fields::VariantF;
	assert_eq!(5,e.index());

	let _ = e.index() as <Fields<i32> as Index>::Type;
}

#[test]
fn test_fields_into_index(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert_eq!(0,e.into_index());

	e = Fields::VariantB(0);
	assert_eq!(1,e.into_index());

	e = Fields::VariantC(0,1,2,3);
	assert_eq!(2,e.into_index());

	e = Fields::VariantD{d: 0};
	assert_eq!(3,e.into_index());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert_eq!(4,e.into_index());

	e = Fields::VariantF;
	assert_eq!(5,e.into_index());
}

#[test]
fn test_nofields_from_index(){
	assert_eq!(NoFields::A,NoFields::from_index(0).unwrap());
	assert_eq!(NoFields::B,NoFields::from_index(1).unwrap());
	assert_eq!(NoFields::C,NoFields::from_index(2).unwrap());
	assert_eq!(NoFields::D,NoFields::from_index(3).unwrap());
	assert_eq!(NoFields::E,NoFields::from_index(4).unwrap());
	assert_eq!(NoFields::F,NoFields::from_index(5).unwrap());
}

#[test]
fn test_nofields_index(){
	let _ = NoFields::E.index() as <Fields<i32> as Index>::Type;
}

#[test]
fn test_discriminants(){
	assert_eq!(None                  ,Discriminants::from_discriminant(0));
	assert_eq!(Some(Discriminants::A),Discriminants::from_discriminant(1));
	assert_eq!(Some(Discriminants::B),Discriminants::from_discriminant(2));
	assert_eq!(None                  ,Discriminants::from_discriminant(3));
	assert_eq!(Some(Discriminants::C),Discriminants::from_discriminant(4));
	assert_eq!(Some(Discriminants::D),Discriminants::from_discriminant(8));
	assert_eq!(Some(Discriminants::E),Discriminants::from_discriminant(16));
	assert_eq!(None                  ,Discriminants::from_discriminant(32));
	assert_eq!(Some(Discriminants::F),Discriminants::from_discriminant(33));

	assert_eq!(Some(SomeDiscriminants::A),SomeDiscriminants::from_discriminant(SomeDiscriminants::A as u32));
	assert_eq!(Some(SomeDiscriminants::B),SomeDiscriminants::from_discriminant(SomeDiscriminants::B as u32));
	assert_eq!(Some(SomeDiscriminants::C),SomeDiscriminants::from_discriminant(SomeDiscriminants::C as u32));
	assert_eq!(Some(SomeDiscriminants::D),SomeDiscriminants::from_discriminant(SomeDiscriminants::D as u32));
	assert_eq!(Some(SomeDiscriminants::E),SomeDiscriminants::from_discriminant(SomeDiscriminants::E as u32));
	assert_eq!(Some(SomeDiscriminants::F),SomeDiscriminants::from_discriminant(SomeDiscriminants::F as u32));

	assert_eq!(Some(NoFields::A),NoFields::from_discriminant(NoFields::A as usize));
	assert_eq!(Some(NoFields::B),NoFields::from_discriminant(NoFields::B as usize));
	assert_eq!(Some(NoFields::C),NoFields::from_discriminant(NoFields::C as usize));
	assert_eq!(Some(NoFields::D),NoFields::from_discriminant(NoFields::D as usize));
	assert_eq!(Some(NoFields::E),NoFields::from_discriminant(NoFields::E as usize));
	assert_eq!(Some(NoFields::F),NoFields::from_discriminant(NoFields::F as usize));
}


#[allow(dead_code)]
#[test]
fn test_len(){
	assert_eq!(6,Fields::<'static,u32>::LEN);
	assert_eq!(6,NoFields::LEN);
	assert_eq!(6,Discriminants::LEN);

	{
		#[derive(EnumLen)]enum T{}
		assert_eq!(0,T::LEN);
	}

	{
		#[derive(EnumLen)]enum T{A}
		assert_eq!(1,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C}
		assert_eq!(3,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G}
		assert_eq!(7,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H}
		assert_eq!(8,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(25,T::LEN);
	}

	{
		#[derive(EnumLen)]enum T{A(u32)}
		assert_eq!(1,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64}}
		assert_eq!(3,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G}
		assert_eq!(7,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G,H}
		assert_eq!(8,T::LEN);
	}{
		#[derive(EnumLen)]enum T{A(u32),B,C{a: u64},D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(25,T::LEN);
	}
}

#[allow(dead_code)]
#[test]
fn test_ends(){
	{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A}
		assert_eq!(T::A,T::first());
		assert_eq!(T::A,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C}
		assert_eq!(T::A,T::first());
		assert_eq!(T::C,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G}
		assert_eq!(T::A,T::first());
		assert_eq!(T::G,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H}
		assert_eq!(T::A,T::first());
		assert_eq!(T::H,T::last());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(T::A,T::first());
		assert_eq!(T::Z,T::last());
	}
}


#[allow(dead_code)]
#[test]
fn test_iterator(){
	use std::iter::Iterator;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A}
		let mut t = T::first();
		assert_eq!(T::A,t);        assert_eq!(t.len(),0);
		assert_eq!(None,t.next()); assert_eq!(t.len(),0);

		assert_eq!(t.last(),Some(T::A));
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C}
		let mut t = T::first();

		assert_eq!(T::A,t);              assert_eq!(t.len(),2);
		assert_eq!(Some(T::B),t.next()); assert_eq!(t.len(),1);
		assert_eq!(Some(T::C),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None      ,t.next()); assert_eq!(t.len(),0);

		assert_eq!(t.count(),0);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		let mut t = T::first();
		assert_eq!(T::A,t);
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(Some(T::I),t.next());
		assert_eq!(Some(T::J),t.next());
		assert_eq!(Some(T::K),t.next());
		assert_eq!(Some(T::L),t.next());
		assert_eq!(Some(T::M),t.next());
		assert_eq!(Some(T::N),t.next());
		assert_eq!(Some(T::O),t.next());
		assert_eq!(Some(T::P),t.next());
		assert_eq!(Some(T::Q),t.next());
		assert_eq!(Some(T::R),t.next());
		assert_eq!(Some(T::S),t.next());
		assert_eq!(Some(T::T),t.next());
		assert_eq!(Some(T::U),t.next());
		assert_eq!(Some(T::V),t.next());
		assert_eq!(Some(T::X),t.next());
		assert_eq!(Some(T::Y),t.next());
		assert_eq!(Some(T::Z),t.next());
		assert_eq!(None      ,t.next());
	}
}

#[allow(dead_code)]
#[test]
fn test_iter(){
	use std::iter::Iterator;
	use enum_traits::Iterable;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A}
		let mut t = T::variants();
		assert_eq!(t.len(),1);

		assert_eq!(Some(T::A),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None,t.next());       assert_eq!(t.len(),0);

		assert_eq!(t.last(),Some(T::A));
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C}
		let mut t = T::variants();
		assert_eq!(t.len(),3);

		assert_eq!(Some(T::A),t.next()); assert_eq!(t.len(),2);
		assert_eq!(Some(T::B),t.next()); assert_eq!(t.len(),1);
		assert_eq!(Some(T::C),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None      ,t.next()); assert_eq!(t.len(),0);

		assert_eq!(t.count(),0);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G,H}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(None      ,t.next());
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIter)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		let mut t = T::variants();
		assert_eq!(Some(T::A),t.next());
		assert_eq!(Some(T::B),t.next());
		assert_eq!(Some(T::C),t.next());
		assert_eq!(Some(T::D),t.next());
		assert_eq!(Some(T::E),t.next());
		assert_eq!(Some(T::F),t.next());
		assert_eq!(Some(T::G),t.next());
		assert_eq!(Some(T::H),t.next());
		assert_eq!(Some(T::I),t.next());
		assert_eq!(Some(T::J),t.next());
		assert_eq!(Some(T::K),t.next());
		assert_eq!(Some(T::L),t.next());
		assert_eq!(Some(T::M),t.next());
		assert_eq!(Some(T::N),t.next());
		assert_eq!(Some(T::O),t.next());
		assert_eq!(Some(T::P),t.next());
		assert_eq!(Some(T::Q),t.next());
		assert_eq!(Some(T::R),t.next());
		assert_eq!(Some(T::S),t.next());
		assert_eq!(Some(T::T),t.next());
		assert_eq!(Some(T::U),t.next());
		assert_eq!(Some(T::V),t.next());
		assert_eq!(Some(T::X),t.next());
		assert_eq!(Some(T::Y),t.next());
		assert_eq!(Some(T::Z),t.next());
		assert_eq!(None      ,t.next());
	}
}

#[test]
fn test_variant_name() {
	#[derive(Debug,EnumVariantName)]
	enum Enum {
		Dog,
		Cat(i32),
		Robot{speed: f32},
	}
	assert_eq!(Enum::Dog.variant_name(), "Dog");
	assert_eq!(Enum::Cat(0).variant_name(), "Cat");
	assert_eq!(Enum::Robot{speed: 0.0}.variant_name(), "Robot");
}

#[test]
fn test_from_variant_name() {
	use std::str::FromStr;

	let mut v: Result<Fields<'static,()>,()>;
	assert_eq!({v=Fields::from_str("VariantA"); v},Err(()));
	assert_eq!({v=Fields::from_str("VariantB"); v},Err(()));
	assert_eq!({v=Fields::from_str("VariantC"); v},Err(()));
	assert_eq!({v=Fields::from_str("VariantD"); v},Err(()));
	assert_eq!({v=Fields::from_str("VariantE"); v},Err(()));
	assert_eq!({v=Fields::from_str("VariantF"); v},Ok(Fields::VariantF));

	let mut v: Result<NoFields,()>;
	assert_eq!({v=NoFields::from_str("A"); v},Ok(NoFields::A));
	assert_eq!({v=NoFields::from_str("B"); v},Ok(NoFields::B));
	assert_eq!({v=NoFields::from_str("C"); v},Ok(NoFields::C));
	assert_eq!({v=NoFields::from_str("D"); v},Ok(NoFields::D));
	assert_eq!({v=NoFields::from_str("E"); v},Ok(NoFields::E));
	assert_eq!({v=NoFields::from_str("F"); v},Ok(NoFields::F));
}

#[test]
#[allow(dead_code)]
fn f1(){
	#[derive(Debug,EnumIndex,EnumToIndex,EnumLen)]
	enum Enum<'t,T: 't>{
		VariantA(&'t T),
		VariantB(T),
		VariantC(T,T,T),
		VariantD{d: i32},
		VariantE{a: i8,b: i16,c: i32},
		VariantF,
	}

	assert_eq!(Enum::VariantB("OK").into_index(),1);
	assert_eq!(Enum::<'static,&'static str>::LEN,6);
}

#[test]
#[allow(dead_code)]
fn f2(){
	#[derive(Debug,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIter,EnumIterator,EnumDiscriminant,EnumEnds)]
	enum Enum{
		VariantA = 10,
		VariantB = 20,
		VariantC = 30,
	}

	//From EnumToIndex
	assert_eq!(Enum::VariantB.into_index(),1);

	//From EnumLen
	assert_eq!(Enum::LEN,3);

	//From EnumFromIndex
	assert!(match Enum::from_index(1){
		Some(Enum::VariantB) => true,
		_ => false
	});

	//From EnumDiscriminant
	assert!(match Enum::from_discriminant(20){
		Some(Enum::VariantB) => true,
		_ => false
	});

	//From EnumEnds
	assert!(match Enum::first(){
		Enum::VariantA => true,
		_ => false
	});

	//From EnumEnds
	assert!(match <Enum as Ends>::last(){
		Enum::VariantC => true,
		_ => false
	});

	//From EnumIter
	assert!(match Enum::variants().next(){
		Some(Enum::VariantA) => true,
		_ => false
	});

	//From EnumIterator
	assert!(match Enum::VariantA.next(){
		Some(Enum::VariantB) => true,
		_ => false
	});
}

#[derive(EnumIndex,EnumBitPattern)]
#[allow(dead_code,non_camel_case_types)]
enum Enum_u8_1{}

#[derive(EnumIndex,EnumBitPattern)]
#[allow(dead_code,non_camel_case_types)]
enum Enum_u8_2{
	A000,
}

#[derive(EnumIndex,EnumBitPattern)]
#[allow(dead_code,non_camel_case_types)]
enum Enum_u8_3{
	A000,A001,A002,A003,A004,A005,A006,A007,A008,A009,
	A010,A011,A012,A013,A014,A015,A016,A017,A018,A019,
	A020,A021,A022,A023,A024,A025,A026,A027,A028,A029,
	A030,A031,A032,A033,A034,A035,A036,A037,A038,A039,
	A040,A041,A042,A043,A044,A045,A046,A047,A048,A049,
	A050,A051,A052,A053,A054,A055,A056,A057,A058,A059,
	A060,A061,A062,A063,A064,A065,A066,A067,A068,A069,
	A070,A071,A072,A073,A074,A075,A076,A077,A078,A079,
	A080,A081,A082,A083,A084,A085,A086,A087,A088,A089,
	A090,A091,A092,A093,A094,A095,A096,A097,A098,A099,
	A100,A101,A102,A103,A104,A105,A106,A107,A108,A109,
	A110,A111,A112,A113,A114,A115,A116,A117,A118,A119,
	A120,A121,A122,A123,A124,A125,A126,A127,A128,A129,
	A130,A131,A132,A133,A134,A135,A136,A137,A138,A139,
	A140,A141,A142,A143,A144,A145,A146,A147,A148,A149,
	A150,A151,A152,A153,A154,A155,A156,A157,A158,A159,
	A160,A161,A162,A163,A164,A165,A166,A167,A168,A169,
	A170,A171,A172,A173,A174,A175,A176,A177,A178,A179,
	A180,A181,A182,A183,A184,A185,A186,A187,A188,A189,
	A190,A191,A192,A193,A194,A195,A196,A197,A198,A199,
	A200,A201,A202,A203,A204,A205,A206,A207,A208,A209,
	A210,A211,A212,A213,A214,A215,A216,A217,A218,A219,
	A220,A221,A222,A223,A224,A225,A226,A227,A228,A229,
	A230,A231,A232,A233,A234,A235,A236,A237,A238,A239,
	A240,A241,A242,A243,A244,A245,A246,A247,A248,A249,
	A250,A251,A252,A253,A254,A255
}

#[derive(EnumIndex,EnumBitPattern)]
#[allow(dead_code,non_camel_case_types)]
#[repr(u8)]
enum Enum_u8_4{
	A000,
}

#[derive(EnumIndex,EnumBitPattern)]
#[allow(dead_code,non_camel_case_types)]
enum Enum_u16_1{
	A000,A001,A002,A003,A004,A005,A006,A007,A008,A009,
	A010,A011,A012,A013,A014,A015,A016,A017,A018,A019,
	A020,A021,A022,A023,A024,A025,A026,A027,A028,A029,
	A030,A031,A032,A033,A034,A035,A036,A037,A038,A039,
	A040,A041,A042,A043,A044,A045,A046,A047,A048,A049,
	A050,A051,A052,A053,A054,A055,A056,A057,A058,A059,
	A060,A061,A062,A063,A064,A065,A066,A067,A068,A069,
	A070,A071,A072,A073,A074,A075,A076,A077,A078,A079,
	A080,A081,A082,A083,A084,A085,A086,A087,A088,A089,
	A090,A091,A092,A093,A094,A095,A096,A097,A098,A099,
	A100,A101,A102,A103,A104,A105,A106,A107,A108,A109,
	A110,A111,A112,A113,A114,A115,A116,A117,A118,A119,
	A120,A121,A122,A123,A124,A125,A126,A127,A128,A129,
	A130,A131,A132,A133,A134,A135,A136,A137,A138,A139,
	A140,A141,A142,A143,A144,A145,A146,A147,A148,A149,
	A150,A151,A152,A153,A154,A155,A156,A157,A158,A159,
	A160,A161,A162,A163,A164,A165,A166,A167,A168,A169,
	A170,A171,A172,A173,A174,A175,A176,A177,A178,A179,
	A180,A181,A182,A183,A184,A185,A186,A187,A188,A189,
	A190,A191,A192,A193,A194,A195,A196,A197,A198,A199,
	A200,A201,A202,A203,A204,A205,A206,A207,A208,A209,
	A210,A211,A212,A213,A214,A215,A216,A217,A218,A219,
	A220,A221,A222,A223,A224,A225,A226,A227,A228,A229,
	A230,A231,A232,A233,A234,A235,A236,A237,A238,A239,
	A240,A241,A242,A243,A244,A245,A246,A247,A248,A249,
	A250,A251,A252,A253,A254,A255,A256
}

#[derive(EnumIndex)]
#[allow(dead_code,non_camel_case_types)]
enum Enum_u16_2{A = 256}

#[derive(EnumIndex)]
#[allow(dead_code,non_camel_case_types)]
#[repr(u16)]
enum Enum_u16_3{
	A000,
}

#[test]
fn test_index(){
	//Type checking
	let n: <Enum_u8_1 as Index>::Type = 0; n == 0u8;
	let n: <Enum_u8_2 as Index>::Type = 0; n == 0u8;
	let n: <Enum_u8_3 as Index>::Type = 0; n == 0u8;
	let n: <Enum_u8_4 as Index>::Type = 0; n == 0u8;

	let n: <Enum_u16_1 as Index>::Type = 0; n == 0u16;
	let n: <Enum_u16_2 as Index>::Type = 0; n == 0u8;
	let n: <Enum_u16_3 as Index>::Type = 0; n == 0u16;
}

#[test]
fn test_bitpatterns(){
	let _: <Enum_u8_1  as BitPattern>::ByteArray = [0; 0];
	let _: <Enum_u8_2  as BitPattern>::ByteArray = [0; 1];
	let _: <Enum_u16_1 as BitPattern>::ByteArray = [0; 33];

	assert_eq!(Enum_u16_1::A000.bit_pattern_rev()[0..16] , [0b10000000,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A001.bit_pattern_rev()[0..16] , [0b01000000,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A002.bit_pattern_rev()[0..16] , [0b00100000,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A003.bit_pattern_rev()[0..16] , [0b00010000,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A004.bit_pattern_rev()[0..16] , [0b00001000,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A005.bit_pattern_rev()[0..16] , [0b00000100,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A006.bit_pattern_rev()[0..16] , [0b00000010,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A007.bit_pattern_rev()[0..16] , [0b00000001,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A008.bit_pattern_rev()[0..16] , [0,0b10000000,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A009.bit_pattern_rev()[0..16] , [0,0b01000000,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A010.bit_pattern_rev()[0..16] , [0,0b00100000,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A011.bit_pattern_rev()[0..16] , [0,0b00010000,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A012.bit_pattern_rev()[0..16] , [0,0b00001000,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A013.bit_pattern_rev()[0..16] , [0,0b00000100,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A014.bit_pattern_rev()[0..16] , [0,0b00000010,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A015.bit_pattern_rev()[0..16] , [0,0b00000001,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A016.bit_pattern_rev()[0..16] , [0,0,0b10000000,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A017.bit_pattern_rev()[0..16] , [0,0,0b01000000,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A018.bit_pattern_rev()[0..16] , [0,0,0b00100000,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A019.bit_pattern_rev()[0..16] , [0,0,0b00010000,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A020.bit_pattern_rev()[0..16] , [0,0,0b00001000,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A021.bit_pattern_rev()[0..16] , [0,0,0b00000100,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A022.bit_pattern_rev()[0..16] , [0,0,0b00000010,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A023.bit_pattern_rev()[0..16] , [0,0,0b00000001,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A024.bit_pattern_rev()[0..16] , [0,0,0,0b10000000,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A025.bit_pattern_rev()[0..16] , [0,0,0,0b01000000,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A026.bit_pattern_rev()[0..16] , [0,0,0,0b00100000,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A027.bit_pattern_rev()[0..16] , [0,0,0,0b00010000,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A028.bit_pattern_rev()[0..16] , [0,0,0,0b00001000,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A029.bit_pattern_rev()[0..16] , [0,0,0,0b00000100,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A030.bit_pattern_rev()[0..16] , [0,0,0,0b00000010,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A031.bit_pattern_rev()[0..16] , [0,0,0,0b00000001,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A032.bit_pattern_rev()[0..16] , [0,0,0,0,0b10000000,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A033.bit_pattern_rev()[0..16] , [0,0,0,0,0b01000000,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A034.bit_pattern_rev()[0..16] , [0,0,0,0,0b00100000,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A035.bit_pattern_rev()[0..16] , [0,0,0,0,0b00010000,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A036.bit_pattern_rev()[0..16] , [0,0,0,0,0b00001000,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A037.bit_pattern_rev()[0..16] , [0,0,0,0,0b00000100,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A038.bit_pattern_rev()[0..16] , [0,0,0,0,0b00000010,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A039.bit_pattern_rev()[0..16] , [0,0,0,0,0b00000001,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A040.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b10000000,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A041.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b01000000,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A042.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00100000,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A043.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00010000,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A044.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00001000,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A045.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00000100,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A046.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00000010,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A047.bit_pattern_rev()[0..16] , [0,0,0,0,0,0b00000001,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A048.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b10000000,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A049.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b01000000,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A050.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00100000,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A051.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00010000,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A052.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00001000,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A053.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00000100,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A054.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00000010,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A055.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0b00000001,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A056.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b10000000 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A057.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b01000000 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A058.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00100000 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A059.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00010000 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A060.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00001000 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A061.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00000100 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A062.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00000010 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A063.bit_pattern_rev()[0..16] , [0,0,0,0,0,0,0,0b00000001 , 0,0,0,0,0,0,0,0]);

	assert_eq!(Enum_u16_1::A000.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A001.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A002.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A003.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A004.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A005.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A006.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A007.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A008.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A009.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A010.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A011.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A012.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A013.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A014.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A015.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A016.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A017.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A018.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A019.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A020.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A021.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A022.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A023.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A024.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A025.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A026.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A027.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A028.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A029.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A030.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A031.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A032.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A033.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A034.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A035.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A036.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A037.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A038.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A039.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A040.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A041.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A042.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A043.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A044.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A045.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A046.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A047.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A048.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A049.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A050.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A051.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A052.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A053.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A054.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A055.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A056.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A057.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A058.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A059.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A060.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A061.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A062.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);
	assert_eq!(Enum_u16_1::A063.bit_pattern_rev()[16..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0]);



	assert_eq!(Enum_u16_1::A000.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A001.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A002.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A003.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A004.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A005.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A006.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A007.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A008.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A009.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A010.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A011.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A012.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A013.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A014.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A015.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A016.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A017.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A018.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A019.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A020.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A021.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A022.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A023.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A024.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A025.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A026.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A027.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A028.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A029.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A030.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A031.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A032.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A033.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A034.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A035.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A036.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A037.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A038.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A039.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A040.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A041.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A042.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A043.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A044.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A045.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A046.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A047.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A048.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A049.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A050.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A051.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A052.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A053.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A054.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A055.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A056.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A057.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A058.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A059.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A060.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A061.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A062.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A063.bit_pattern()[0..17] , [0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0]);

	assert_eq!(Enum_u16_1::A000.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00000001]);
	assert_eq!(Enum_u16_1::A001.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00000010]);
	assert_eq!(Enum_u16_1::A002.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00000100]);
	assert_eq!(Enum_u16_1::A003.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00001000]);
	assert_eq!(Enum_u16_1::A004.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00010000]);
	assert_eq!(Enum_u16_1::A005.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b00100000]);
	assert_eq!(Enum_u16_1::A006.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b01000000]);
	assert_eq!(Enum_u16_1::A007.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0b10000000]);
	assert_eq!(Enum_u16_1::A008.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00000001,0]);
	assert_eq!(Enum_u16_1::A009.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00000010,0]);
	assert_eq!(Enum_u16_1::A010.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00000100,0]);
	assert_eq!(Enum_u16_1::A011.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00001000,0]);
	assert_eq!(Enum_u16_1::A012.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00010000,0]);
	assert_eq!(Enum_u16_1::A013.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b00100000,0]);
	assert_eq!(Enum_u16_1::A014.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b01000000,0]);
	assert_eq!(Enum_u16_1::A015.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0b10000000,0]);
	assert_eq!(Enum_u16_1::A016.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00000001,0,0]);
	assert_eq!(Enum_u16_1::A017.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00000010,0,0]);
	assert_eq!(Enum_u16_1::A018.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00000100,0,0]);
	assert_eq!(Enum_u16_1::A019.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00001000,0,0]);
	assert_eq!(Enum_u16_1::A020.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00010000,0,0]);
	assert_eq!(Enum_u16_1::A021.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b00100000,0,0]);
	assert_eq!(Enum_u16_1::A022.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b01000000,0,0]);
	assert_eq!(Enum_u16_1::A023.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0,0b10000000,0,0]);
	assert_eq!(Enum_u16_1::A024.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00000001,0,0,0]);
	assert_eq!(Enum_u16_1::A025.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00000010,0,0,0]);
	assert_eq!(Enum_u16_1::A026.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00000100,0,0,0]);
	assert_eq!(Enum_u16_1::A027.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00001000,0,0,0]);
	assert_eq!(Enum_u16_1::A028.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00010000,0,0,0]);
	assert_eq!(Enum_u16_1::A029.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b00100000,0,0,0]);
	assert_eq!(Enum_u16_1::A030.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b01000000,0,0,0]);
	assert_eq!(Enum_u16_1::A031.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0,0b10000000,0,0,0]);
	assert_eq!(Enum_u16_1::A032.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00000001,0,0,0,0]);
	assert_eq!(Enum_u16_1::A033.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00000010,0,0,0,0]);
	assert_eq!(Enum_u16_1::A034.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00000100,0,0,0,0]);
	assert_eq!(Enum_u16_1::A035.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00001000,0,0,0,0]);
	assert_eq!(Enum_u16_1::A036.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00010000,0,0,0,0]);
	assert_eq!(Enum_u16_1::A037.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b00100000,0,0,0,0]);
	assert_eq!(Enum_u16_1::A038.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b01000000,0,0,0,0]);
	assert_eq!(Enum_u16_1::A039.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0,0b10000000,0,0,0,0]);
	assert_eq!(Enum_u16_1::A040.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00000001,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A041.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00000010,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A042.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00000100,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A043.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00001000,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A044.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00010000,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A045.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b00100000,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A046.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b01000000,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A047.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0,0b10000000,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A048.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00000001,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A049.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00000010,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A050.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00000100,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A051.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00001000,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A052.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00010000,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A053.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b00100000,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A054.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b01000000,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A055.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0,0b10000000,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A056.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00000001,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A057.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00000010,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A058.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00000100,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A059.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00001000,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A060.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00010000,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A061.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b00100000,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A062.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b01000000,0,0,0,0,0,0,0]);
	assert_eq!(Enum_u16_1::A063.bit_pattern()[17..33] , [0,0,0,0,0,0,0,0 , 0b10000000,0,0,0,0,0,0,0]);
}

#[test]
fn test_tag(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert_eq!(FieldsTag::VariantA,e.tag());

	e = Fields::VariantB(0);
	assert_eq!(FieldsTag::VariantB,e.tag());

	e = Fields::VariantC(0,1,2,3);
	assert_eq!(FieldsTag::VariantC,e.tag());

	e = Fields::VariantD{d: 0};
	assert_eq!(FieldsTag::VariantD,e.tag());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert_eq!(FieldsTag::VariantE,e.tag());

	e = Fields::VariantF;
	assert_eq!(FieldsTag::VariantF,e.tag());
}

#[test]
fn test_isvariantfns(){
	let i = 0u8;
	let mut e = Fields::VariantA(&i);
	assert!(e.is_varianta());
	assert!(!e.is_variantb());
	assert!(!e.is_variantc());
	assert!(!e.is_variantd());
	assert!(!e.is_variante());
	assert!(!e.is_variantf());

	e = Fields::VariantB(0);
	assert!(!e.is_varianta());
	assert!(e.is_variantb());
	assert!(!e.is_variantc());
	assert!(!e.is_variantd());
	assert!(!e.is_variante());
	assert!(!e.is_variantf());

	e = Fields::VariantC(0,1,2,3);
	assert!(!e.is_varianta());
	assert!(!e.is_variantb());
	assert!(e.is_variantc());
	assert!(!e.is_variantd());
	assert!(!e.is_variante());
	assert!(!e.is_variantf());

	e = Fields::VariantD{d: 0};
	assert!(!e.is_varianta());
	assert!(!e.is_variantb());
	assert!(!e.is_variantc());
	assert!(e.is_variantd());
	assert!(!e.is_variante());
	assert!(!e.is_variantf());

	e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	assert!(!e.is_varianta());
	assert!(!e.is_variantb());
	assert!(!e.is_variantc());
	assert!(!e.is_variantd());
	assert!(e.is_variante());
	assert!(!e.is_variantf());

	e = Fields::VariantF;
	assert!(!e.is_varianta());
	assert!(!e.is_variantb());
	assert!(!e.is_variantc());
	assert!(!e.is_variantd());
	assert!(!e.is_variante());
	assert!(e.is_variantf());
}
