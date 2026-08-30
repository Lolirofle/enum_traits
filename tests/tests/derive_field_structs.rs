#![allow(non_camel_case_types)]

use core::convert::TryFrom;
use enum_traits_macros::*;

#[test]
fn from_unit(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A}

	assert_eq!(E::from(A),E::A);
}

#[test]
fn from_tuple(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A(i8,i16,i32)}

	assert_eq!(E::from(A(1,2,3)),E::A(1,2,3));
}

#[test]
fn from_record(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A{x: i8,y: i16}}

	assert_eq!(E::from(A{x: 1,y: 2}),E::A{x: 1,y: 2});
}

#[test]
fn from_single(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A(i8)}

	assert_eq!(E::from(A(1)),E::A(1));
}

#[test]
fn from_empty_tuple(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A(())}

	assert_eq!(E::from(A(())),E::A(()));
}

#[test]
fn from_multiple(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		A(i8),
		B(i16,i32),
		C{x: i8},
	}

	assert_eq!(E::from(A(1)),E::A(1));
	assert_eq!(E::from(B(2,3)),E::B(2,3));
	assert_eq!(E::from(C{x: 4}),E::C{x: 4});
}

#[test]
fn from_generics(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E<'a,X,Y>{
		G(X),
		H(&'a Y),
		Both(X,&'a Y),
		Nope,
	}

	let y = 5i32;
	let _: E<'_,u8,i32> = E::from(G(1u8));
	let _: E<'_,u8,i32> = E::from(H(&y));
	let _: E<'_,u8,i32> = E::from(Both(1u8,&y));
	let _: E<'_,u8,i32> = E::from(Nope);
}

#[test]
fn from_generics_order(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E<'a,T,U>{
		V(U,T,&'a str),
	}

	let s = "x";
	let v: E<'_,u8,u16> = E::from(V(1u16,2u8,&s));
	assert_eq!(v,E::V(1u16,2u8,&s));
}

#[test]
fn from_const_generic(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E<const N: usize>{
		A([u8; N]),
		B,
	}

	let _: E<3> = E::from(A([0u8; 3]));
	let _: E<3> = E::from(B);
}

#[test]
fn from_where_bounds(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E<T> where
		T: Clone
	{
		A(T),
	}

	let v: E<u8> = E::from(A(1u8));
	assert_eq!(v,E::A(1));
}

#[test]
fn try_from_simple(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] A))] A(i8),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] B))] B(i16)
	}

	assert_eq!(A::try_from(E::A(5)).unwrap(),A(5));
	assert_eq!(B::try_from(E::B(7)).unwrap(),B(7));
}

#[test]
fn try_from_err(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{A(i8),B(i16)}

	assert!(A::try_from(E::B(1)).is_err());
	assert!(B::try_from(E::A(1)).is_err());
}

#[test]
fn from_try_from_inverse(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum Z<'a,X,Y>{
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] A))] A(i8),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] B))] B(i32),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] C))] C(u8,u16,u32),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] D))] D{d: (u8,i32)},
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] E))] E{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] F))] F,
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] G))] G(X),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] H))] H(&'a Y),
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] I))] I{x: X,y: &'a Y},
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] J))] J{x: i8},
	}

	let e = Z::<'static,u64,i64>::from(A(1));
	assert_eq!(A::try_from(e).unwrap(),A(1));

	let e = Z::<'static,u64,i64>::from(F);
	assert_eq!(F::try_from(e).unwrap(),F);

	let e = Z::<'static,u64,i64>::from(G(14));
	assert_eq!(G::try_from(e).unwrap(),G(14));

	let e = Z::<'static,u64,i64>::from(H(&15i64));
	let h = H::try_from(e).unwrap();
	assert_eq!(h,H(&15i64));
}

#[test]
fn tuple_name(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(#[derive(Debug,Eq,PartialEq)] RenamedTuple))]
		A(i8),
	}

	assert_eq!(E::from(RenamedTuple(5)),E::A(5));
	assert_eq!(RenamedTuple::try_from(E::A(5)).unwrap(),RenamedTuple(5));
}

#[test]
fn record_name(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(RenamedRecord))]
		A{x: i8,y: i16},
	}

	assert_eq!(E::from(RenamedRecord{x: 1,y: 2}),E::A{x: 1,y: 2});
}

#[test]
fn unit_name(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(RenamedUnit))]
		A,
	}

	assert_eq!(E::from(RenamedUnit),E::A);
}

#[test]
fn struct_attr(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(#[derive(Clone,Debug,PartialEq)] #[derive(Eq)] Renamed))]
		A(i8),
	}

	let r = Renamed(1);
	assert_eq!(r.clone(),Renamed(1));
	assert_eq!(E::from(Renamed(2)),E::A(2));
}

#[test]
fn name_with_visibility(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		#[enum_field_struct(name(pub Renamed))]
		A(i8),
	}

	assert_eq!(E::from(Renamed(1)),E::A(1));
}

#[test]
fn pub_fields(){
	mod inner{
		use super::*;

		#[derive(EnumFieldStruct)]
		pub enum PubE{
			A(u8,u16),
			B{x: u8},
		}
	}
	inner::A(1,2);
	inner::B{x: 3};
}

#[test]
fn reference_field(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E<'a>{
		A(&'a u8),
		B,
	}

	let x = 5u8;
	let _: E<'_> = E::from(A(&x));
	let _: E<'_> = E::from(B);
}

#[test]
fn raw_identifier(){
	#[derive(EnumFieldStruct,Debug,PartialEq,Eq)]
	enum E{
		r#type(i8),
	}

	let _ = E::from(r#type(1));
	let _ = E::r#type(1);
}
