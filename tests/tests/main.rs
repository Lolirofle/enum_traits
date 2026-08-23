#![allow(unreachable_code)]

extern crate alloc;

use enum_traits::*;
use enum_traits_macros::*;

//TODO: Unit tests that should fail
//TODO: Tests for EnumFrom and other stuff that are not tested

mod fields{
	use enum_traits::*;
	use enum_traits_macros::*;

	#[derive(Debug,Eq,PartialEq,EnumIndex,              EnumToIndex,EnumLen,EnumIs,EnumTag,                               EnumVariantName,EnumFromVariantName)]
	enum Fields<'t,T: 't>{
		VariantA(&'t T),
		VariantB(T),
		VariantC(T,T,T,T),
		VariantD{d: (T,i32)},
		VariantE{a: i32,b: i32,c: i32,d: i32,e: (T,i32)},
		VariantF,
	}

	#[test]
	fn test_index(){
		let i = 0u8;
		let mut e;

		e = Fields::VariantA(&i);
		assert_eq!(0,e.index());
		assert_eq!(0,e.into_index());

		e = Fields::VariantB(0);
		assert_eq!(1,e.index());
		assert_eq!(1,e.into_index());

		e = Fields::VariantC(0,1,2,3);
		assert_eq!(2,e.index());
		assert_eq!(2,e.into_index());

		e = Fields::VariantD{d: (0,0)};
		assert_eq!(3,e.index());
		assert_eq!(3,e.into_index());

		e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: (4,4)};
		assert_eq!(4,e.index());
		assert_eq!(4,e.into_index());

		e = Fields::VariantF;
		assert_eq!(5,e.index());
		assert_eq!(5,e.into_index());

		e = Fields::VariantF;
		let _: <Fields<i32> as Index>::Type = e.index();
	}

	#[test]
	fn test_len(){
		assert_eq!(6,Fields::<'static,u32>::LEN);
	}

	#[test]
	fn test_isvariantfns(){
		let i = 0u8;
		let mut e = Fields::VariantA(&i);
		assert!(e.is_variant_a());
		assert!(!e.is_variant_b());
		assert!(!e.is_variant_c());
		assert!(!e.is_variant_d());
		assert!(!e.is_variant_e());
		assert!(!e.is_variant_f());

		e = Fields::VariantB(0);
		assert!(!e.is_variant_a());
		assert!(e.is_variant_b());
		assert!(!e.is_variant_c());
		assert!(!e.is_variant_d());
		assert!(!e.is_variant_e());
		assert!(!e.is_variant_f());

		e = Fields::VariantC(0,1,2,3);
		assert!(!e.is_variant_a());
		assert!(!e.is_variant_b());
		assert!(e.is_variant_c());
		assert!(!e.is_variant_d());
		assert!(!e.is_variant_e());
		assert!(!e.is_variant_f());

		e = Fields::VariantD{d: (0,0)};
		assert!(!e.is_variant_a());
		assert!(!e.is_variant_b());
		assert!(!e.is_variant_c());
		assert!(e.is_variant_d());
		assert!(!e.is_variant_e());
		assert!(!e.is_variant_f());

		e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: (4,4)};
		assert!(!e.is_variant_a());
		assert!(!e.is_variant_b());
		assert!(!e.is_variant_c());
		assert!(!e.is_variant_d());
		assert!(e.is_variant_e());
		assert!(!e.is_variant_f());

		e = Fields::VariantF;
		assert!(!e.is_variant_a());
		assert!(!e.is_variant_b());
		assert!(!e.is_variant_c());
		assert!(!e.is_variant_d());
		assert!(!e.is_variant_e());
		assert!(e.is_variant_f());
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

		e = Fields::VariantD{d: (0,0)};
		assert_eq!(FieldsTag::VariantD,e.tag());

		e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: (4,4)};
		assert_eq!(FieldsTag::VariantE,e.tag());

		e = Fields::VariantF;
		assert_eq!(FieldsTag::VariantF,e.tag());
	}

	#[test]
	fn test_variant_name() {
		let i = 0u8;
		let mut e;

		e = Fields::VariantA(&i);
		assert_eq!("VariantA",e.variant_name());

		e = Fields::VariantB(0);
		assert_eq!("VariantB",e.variant_name());

		e = Fields::VariantC(0,1,2,3);
		assert_eq!("VariantC",e.variant_name());

		e = Fields::VariantD{d: (0,0)};
		assert_eq!("VariantD",e.variant_name());

		e = Fields::VariantE{a: 0,b: 1,c: 2,d: 3,e: (4,4)};
		assert_eq!("VariantE",e.variant_name());

		e = Fields::VariantF;
		assert_eq!("VariantF",e.variant_name());
	}
}

mod nofields{
	use enum_traits::*;
	use enum_traits_macros::*;

	#[derive(Debug,Eq,PartialEq,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIs,EnumTag,EnumEnds,EnumIterator,EnumIterable,EnumVariantName,EnumFromVariantName)]
	enum NoFields{
		A,B,C,D,E,F
	}

	#[test]
	fn test_index(){
		let _ = NoFields::E.index() as <NoFields as Index>::Type;
	}

	#[test]
	fn test_from_index(){
		assert_eq!(Some(NoFields::A),NoFields::from_index(0));
		assert_eq!(Some(NoFields::B),NoFields::from_index(1));
		assert_eq!(Some(NoFields::C),NoFields::from_index(2));
		assert_eq!(Some(NoFields::D),NoFields::from_index(3));
		assert_eq!(Some(NoFields::E),NoFields::from_index(4));
		assert_eq!(Some(NoFields::F),NoFields::from_index(5));
	}

	#[test]
	fn test_to_index(){
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
	fn test_len(){
		assert_eq!(6,<NoFields as Len>::LEN);
	}

	#[test]
	fn test_variant_name(){
		assert_eq!(NoFields::A.variant_name(),"A");
		assert_eq!(NoFields::B.variant_name(),"B");
		assert_eq!(NoFields::C.variant_name(),"C");
		assert_eq!(NoFields::D.variant_name(),"D");
		assert_eq!(NoFields::E.variant_name(),"E");
		assert_eq!(NoFields::F.variant_name(),"F");
	}

	#[test]
	fn test_from_str(){
		use core::str::FromStr;

		let mut v: Result<NoFields,()>;
		assert_eq!({v=NoFields::from_str("A"); v},Ok(NoFields::A));
		assert_eq!({v=NoFields::from_str("B"); v},Ok(NoFields::B));
		assert_eq!({v=NoFields::from_str("C"); v},Ok(NoFields::C));
		assert_eq!({v=NoFields::from_str("D"); v},Ok(NoFields::D));
		assert_eq!({v=NoFields::from_str("E"); v},Ok(NoFields::E));
		assert_eq!({v=NoFields::from_str("F"); v},Ok(NoFields::F));
	}
}

mod discriminants{
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
	fn test_len(){
		assert_eq!(6,<Discriminants as Len>::LEN);
		assert_eq!(6,<SomeDiscriminants as Len>::LEN);
	}
}

mod large{
	use enum_traits::*;
	use enum_traits_macros::*;

	#[derive(EnumIndex)]
	#[allow(dead_code,non_camel_case_types)]
	enum Enum_u8_1{}//TODO: Test more derives with this

	#[derive(EnumIndex)]
	#[allow(dead_code,non_camel_case_types)]
	enum Enum_u8_2{
		A000,
	}

	#[derive(EnumIndex)]
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

	#[derive(EnumIndex)]
	#[allow(dead_code,non_camel_case_types)]
	#[repr(u8)]
	enum Enum_u8_4{
		A000,
	}

	#[derive(EnumIndex)]
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
		let n: <Enum_u8_1 as Index>::Type = 0; let _ = n == 0u8;
		let n: <Enum_u8_2 as Index>::Type = 0; let _ = n == 0u8;
		let n: <Enum_u8_3 as Index>::Type = 0; let _ = n == 0u8;
		let n: <Enum_u8_4 as Index>::Type = 0; let _ = n == 0u8;

		let n: <Enum_u16_1 as Index>::Type = 0; let _ = n == 0u16;
		let n: <Enum_u16_2 as Index>::Type = 0; let _ = n == 0u8;
		let n: <Enum_u16_3 as Index>::Type = 0; let _ = n == 0u8;
	}
}

mod readmemd{
	use enum_traits::*;
	use enum_traits_macros::*;

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
		#[derive(Debug,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIterable,EnumIterator,EnumEnds)]
		enum Enum{
			VariantA = 10,
			VariantB = 20,
			VariantC = 30,
		}

		//From EnumToIndex
		assert_eq!(Enum::VariantB.into_index(),1);

		//From EnumLen
		//assert_eq!(Enum::LEN,3);
		assert_eq!(<Enum as Len>::LEN,3);

		//From EnumFromIndex
		assert!(match Enum::from_index(1){
			Some(Enum::VariantB) => true,
			_ => false
		});

		//From EnumEnds
		assert!(match Enum::FIRST{
			Enum::VariantA => true,
			_ => false
		});

		//From EnumEnds
		assert!(match <Enum as Ends>::LAST{
			Enum::VariantC => true,
			_ => false
		});

		//From EnumIterable
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
}

mod generic_where{
	use alloc::boxed::Box;
	use alloc::vec::Vec;
	use enum_traits_macros::*;

	#[derive(Debug,Eq,PartialEq,EnumIndex,EnumToIndex,EnumLen,EnumIs,EnumTag,EnumVariantName,EnumFromVariantName)]
	#[allow(dead_code)]
	enum Generic<'x,'y: 'x,'z,X,Y: 'y,Z> where
		'z: 'y,
		X: 'x,
		Y: Iterator,
		<Y as Iterator>::Item: core::fmt::Debug + Clone + Eq,
		u32: From<<Y as Iterator>::Item>,
	{
		A,
		B(&'x X),
		C(Box<X>,Vec<Y>),
		D{y: Vec<Y>,z: &'z Z},
		E{x: Vec<X>,y: &'y Y,z: Box<Z>},
		F{i: i32,z: &'z Z},
		G(u32,i32,u8),
		H{i: i32,u: u32},
		I(<Y as Iterator>::Item),
	}
}

#[test]
fn test_ends(){
	{
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::A,T::LAST);
	}{
		#[allow(dead_code)]
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::C,T::LAST);
	}{
		#[allow(dead_code)]
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::G,T::LAST);
	}{
		#[allow(dead_code)]
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::H,T::LAST);
	}{
		#[allow(dead_code)]
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::Z,T::LAST);
	}{
		#[allow(dead_code)]
		#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B(u64),C{c: i32},D}
		assert_eq!(T::A,T::FIRST);
		assert_eq!(T::D,T::LAST);
	}
}


#[test]
fn test_iterator(){
	use core::iter::Iterator;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A}
		let mut t = T::FIRST;
		assert_eq!(T::A,t);        assert_eq!(t.len(),0);
		assert_eq!(None,t.next()); assert_eq!(t.len(),0);

		assert_eq!(T::LAST,T::A);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C}
		let mut t = T::FIRST;

		assert_eq!(T::A,t);              assert_eq!(t.len(),2);
		assert_eq!(Some(T::B),t.next()); assert_eq!(t.len(),1);
		assert_eq!(Some(T::C),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None      ,t.next()); assert_eq!(t.len(),0);

		assert_eq!(t.count(),0);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterator)]enum T{A,B,C,D,E,F,G}
		let mut t = T::FIRST;
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
		let mut t = T::FIRST;
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
		let mut t = T::FIRST;
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

#[test]
fn test_iter(){
	use core::iter::Iterator;
	use enum_traits::Iterable;

	{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterable)]enum T{A}
		let mut t = T::variants();
		assert_eq!(t.len(),1);

		assert_eq!(Some(T::A),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None,t.next());       assert_eq!(t.len(),0);

		assert_eq!(T::LAST,T::A);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterable)]enum T{A,B,C}
		let mut t = T::variants();
		assert_eq!(t.len(),3);

		assert_eq!(Some(T::A),t.next()); assert_eq!(t.len(),2);
		assert_eq!(Some(T::B),t.next()); assert_eq!(t.len(),1);
		assert_eq!(Some(T::C),t.next()); assert_eq!(t.len(),0);
		assert_eq!(None      ,t.next()); assert_eq!(t.len(),0);

		assert_eq!(t.count(),0);
	}{
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterable)]enum T{A,B,C,D,E,F,G}
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
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterable)]enum T{A,B,C,D,E,F,G,H}
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
		#[derive(Debug,Eq,PartialEq,EnumEnds,EnumIterable)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
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

mod attr{
	#[test]
	fn simple(){
		use enum_traits_macros::*;

		#[allow(dead_code)]
		#[impl_enum_len(LENGTH_NAME)]
		#[impl_enum_first(FIRST_NAME)]
		#[impl_enum_last(LAST_NAME)]
		#[impl_enum_variants_array(VARIANTSSS)]
		#[derive(Eq,PartialEq,Debug)]
		enum T{A,B,C,D}

		assert_eq!(T::LENGTH_NAME,4);
		assert_eq!(T::FIRST_NAME,T::A);
		assert_eq!(T::LAST_NAME,T::D);
		assert_eq!(T::VARIANTSSS.len(),4);
		assert_eq!(T::VARIANTSSS,[T::A,T::B,T::C,T::D]);
	}

	#[test]
	fn visibility(){
		mod inner{
			use enum_traits_macros::*;

			#[allow(dead_code)]
			#[impl_enum_len(pub(crate) LENGTH)]
			#[derive(Eq,PartialEq,Debug)]
			pub enum T{A,B,C,D}
		}

		assert_eq!(inner::T::LENGTH,4);
	}
}
/*
#[test]
fn test_from_discriminants(){
	use enum_traits::*;
	use enum_traits_macros::*;

	#[derive(Debug,Eq,PartialEq,EnumFromDiscriminant)]enum T{
		A = 5,
		B = 1,
		C = 3,
		D = 7,
		//E = 7,
	}

	impl IntoDiscriminant<u8> for T{
		fn into_discriminant(self) -> u8{
			self as u8
		}
	}

	assert_eq!(Some(T::A),t.next());
}
*/

fn main(){}
