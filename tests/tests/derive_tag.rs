#![allow(non_camel_case_types,unused)]

use enum_traits::*;
use enum_traits_macros::*;
use core::assert_matches;

#[test]
fn into_tag(){
	#[derive(EnumTag)]
	enum E{
		A,
		B(i32),
		C{x: u8},
	}

	assert_eq!(E::A.into_tag(),ETag::A);
	assert_eq!(E::B(1).into_tag(),ETag::B);
	assert_eq!(E::C{x: 2}.into_tag(),ETag::C);
}

#[test]
fn tag(){
	#[derive(EnumTag)]
	enum E{
		A,
		B(i32),
	}

	let e = E::B(7);
	assert_eq!(e.tag(),ETag::B);
	assert_eq!(e.into_tag(),ETag::B);
}

#[test]
fn tag_ref(){
	#[derive(EnumTag)]
	enum E{
		A,
		B(i32),
	}

	let variants = [E::A,E::B(1)];
	for v in &variants{
		let _ = v.tag();
	}
}

#[test]
fn tag_match_into_tag(){
	#[derive(EnumTag,Clone)]
	enum E{
		A,
		B(i32),
		C{x: u8},
	}

	for e in [E::A,E::B(1),E::C{x: 0}]{
		assert_eq!(e.clone().into_tag(),e.tag());
	}
}

#[test]
fn tag_default_derives(){
	#[derive(EnumTag)]
	enum E{A,B}

	let x: ETag = ETag::A;
	let y = x;
	let _ = x;
	let _ = y;

	//Clone
	let _ = ETag::A.clone();

	//PartialEq/Eq
	assert_eq!(ETag::A,ETag::A);
	assert_ne!(ETag::A,ETag::B);

	//Hash
	use std::collections::HashSet;
	let mut set = HashSet::new();
	set.insert(ETag::A);
	set.insert(ETag::B);
	set.insert(ETag::A);
	assert_eq!(set.len(),2);
}

#[test]
fn vis(){
	mod inner{
		use super::*;

		#[derive(EnumTag)]
		pub enum PubE{
			A,
			B(u8),
		}

		#[derive(EnumTag)]
		pub(crate) enum PubCrateE{
			A,
			B,
		}

		pub fn make() -> (PubE,PubCrateE){
			(PubE::A,PubCrateE::A)
		}
	}
	let _: inner::PubETag = inner::PubE::A.tag();
	let _: inner::PubCrateETag = inner::PubCrateE::A.tag();
	let (a,b) = inner::make();
	let _: inner::PubETag = a.tag();
	let _: inner::PubCrateETag = b.tag();
}

#[test]
fn custom_name(){
	#[derive(EnumTag)]
	#[enum_tag(name(#[derive(Debug)] Renamed))]
	enum E{
		A,
		B(u8),
	}

	assert_matches!(E::A.tag(),Renamed::A);
	assert_matches!(E::B(0).tag(),Renamed::B);
	assert_matches!(E::A.into_tag(),Renamed::A);
}

#[test]
fn custom_name_attrs(){
	#[derive(EnumTag)]
	#[enum_tag(name(#[derive(Debug,Eq,PartialEq)] Renamed))]
	enum E{
		A,
		B,
	}

	assert_eq!(format!("{:?}",E::A.tag()),"A");
	assert_eq!(E::A.tag(),Renamed::A);
}

mod vis_mod{
	use super::*;

	#[derive(EnumTag)]
	#[enum_tag(name(pub CustomTag))]
	enum E{
		A,
	}

	pub fn tag_of() -> CustomTag{
		E::A.tag()
	}
}

#[test]
fn custom_name_vis(){
	let _: vis_mod::CustomTag = vis_mod::tag_of();
}

#[test]
fn full_override(){
	#![allow(deprecated)]

	#[derive(EnumTag)]
	#[enum_tag(name(#[deprecated] #[derive(Debug,Eq,PartialEq)] pub(crate) EnumHasACustomName))]
	enum Enum{
		A,
		B,
	}

	assert_eq!(Enum::A.tag(),EnumHasACustomName::A);
	assert_eq!(Enum::B.tag(),EnumHasACustomName::B);
}

#[test]
fn omit_copy(){
	#[derive(EnumTag)]
	#[enum_tag(name(#[derive(Debug,Eq,PartialEq,Clone)] NotCopy))]
	enum E{
		A,
		B,
	}

	//`tag(&self) -> Self::Tag` still works without Copy.
	assert_eq!(E::A.tag(),NotCopy::A);
}

#[test]
fn generic(){
	#[derive(EnumTag)]
	enum E<T>{
		A(T),
		B(T,T),
		Nothing,
	}

	assert_eq!(E::<u8>::A(0).tag(),ETag::A);
	assert_eq!(E::<String>::B("".into(),"".into()).tag(),ETag::B);
	assert_eq!(E::<u8>::Nothing.tag(),ETag::Nothing);
}

#[test]
fn generic_custom_name(){
	#[derive(EnumTag)]
	#[enum_tag(name(#[derive(Debug)] Kind))]
	enum E<T>{
		A(T),
	}

	assert_matches!(E::<u8>::A(1).tag(),Kind::A);
}

#[test]
fn where_bounds(){
	#[derive(EnumTag)]
	enum E<T> where
		T: Clone + std::fmt::Debug
   {
		A(T),
		B,
	}

	assert_eq!(E::<u8>::A(0).tag(),ETag::A);
	assert_eq!(E::<u8>::B.tag(),ETag::B);
}

#[test]
fn lifetime_parameters(){
	#[derive(EnumTag)]
	enum E<'a>{
		A(&'a str),
		B,
	}

	assert_eq!(E::A("x").tag(),ETag::A);
	assert_eq!(E::B.tag(),ETag::B);
}

#[test]
fn lifetime_generic_const(){
	#[derive(EnumTag)]
	enum E<'a,T,const N: usize>{
		A([u8; N],&'a T),
		B,
	}

	assert_eq!(E::<'_,i32,3>::A([0; 3],&5i32).tag(),ETag::A);
	assert_eq!(E::<'_,i32,3>::B.tag(),ETag::B);
}

#[test]
fn single_unit(){
	#[derive(EnumTag)]
	enum E{A}

	assert_eq!(E::A.tag(),ETag::A);
	assert_eq!(E::A.into_tag(),ETag::A);
}

#[test]
fn three_units(){
	#[derive(EnumTag)]
	enum E{A,B,C}

	assert_eq!(E::A.tag(),ETag::A);
	assert_eq!(E::B.tag(),ETag::B);
	assert_eq!(E::C.tag(),ETag::C);
}

#[test]
fn three_tuples(){
	#[derive(EnumTag)]
	enum E{
		A(u8),
		B(u8,u16),
		C(u8,u16,u32),
	}

	assert_eq!(E::A(1).tag(),ETag::A);
	assert_eq!(E::B(1,2).tag(),ETag::B);
	assert_eq!(E::C(1,2,3).tag(),ETag::C);
}

#[test]
fn two_records(){
	#[derive(EnumTag)]
	enum E{
		A{x: u8},
		B{x: u8,y: u16},
	}

	assert_eq!(E::A{x: 1}.tag(),ETag::A);
	assert_eq!(E::B{x: 1,y: 2}.tag(),ETag::B);
}

#[test]
fn raw_identifiers(){
	#[derive(EnumTag)]
	enum E{
		r#type,
		r#loop(u8),
	}

	assert_eq!(E::r#type.tag(),ETag::r#type);
	assert_eq!(E::r#loop(1).tag(),ETag::r#loop);
}

#[test]
fn recursive(){
	#[derive(EnumTag)]
	enum E{
		Leaf,
		Node(Box<E>),
	}

	assert_eq!(E::Leaf.tag(),ETag::Leaf);
	assert_eq!(E::Node(Box::new(E::Leaf)).tag(),ETag::Node);
}
