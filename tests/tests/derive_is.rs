#![allow(dead_code,non_camel_case_types)]
use enum_traits_macros::*;

#[derive(EnumIs)]
enum Empty{}

mod naming{
	use super::*;

	#[derive(EnumIs)]
	enum E{
		A,
		Ab,
		AbCd,
		ABC,
		HTTPError,
		XmlHttpRequest,
		A1,
		A2A,
		already_snake,
	}
	#[test] fn single_letter()           {assert!(E::A.is_a());}
	#[test] fn two_letters()             {assert!(E::Ab.is_ab());}
	#[test] fn camel_case()              {assert!(E::AbCd.is_ab_cd());}
	#[test] fn all_caps()                {assert!(E::ABC.is_abc());}
	//#[test] fn acronym_prefix()         {assert!(E::HTTPError.is_http_error());}
	#[test] fn mixed_acronyms()          {assert!(E::XmlHttpRequest.is_xml_http_request());}
	#[test] fn digit_suffix()            {assert!(E::A1.is_a1());}
	#[test] fn digit_in_middle()         {assert!(E::A2A.is_a2_a());}
	#[test] fn already_snake_style_name(){assert!(E::already_snake.is_already_snake());}
	#[test] fn naming_negatives(){
		assert!(!E::A.is_ab());
		assert!(!E::AbCd.is_ab());
		assert!(!E::ABC.is_ab_cd());
	}
}


#[test] fn raw_ident(){
	#[derive(EnumIs)]
	enum E{
		r#type,
		r#loop,
	}
	assert!(E::r#type.is_type());
	assert!(E::r#loop.is_loop());
}

mod kinds{
	use super::*;

	#[derive(EnumIs)]
	enum E<'l,T,const N: usize>{
		Unit,
		Tuple(i32,u32),
		One(&'l T),
		Struct{a: i8,b: i16},
		OnlyField{x: [i8; N]},
	}

	#[test]
	fn kinds_positive(){
		assert!(E::Unit     ::<'static,isize,2>.is_unit());
		assert!(E::Tuple    ::<'static,isize,2>(1,2).is_tuple());
		assert!(E::One      ::<'static,isize,2>(&1).is_one());
		assert!(E::Struct   ::<'static,isize,2>{a: 1,b: 2}.is_struct());
		assert!(E::OnlyField::<'static,isize,2>{x: [1,2]}.is_only_field());
	}

	#[test]
	fn kinds_negative(){
		assert!(!E::Unit  ::<'static,isize,2>.is_tuple());
		assert!(!E::Unit  ::<'static,isize,2>.is_struct());
		assert!(!E::Tuple ::<'static,isize,2>(1,2).is_unit());
		assert!(!E::Struct::<'static,isize,2>{a: 1,b: 2}.is_unit());
		assert!(!E::One   ::<'static,isize,2>(&1).is_tuple());
		assert!(!E::Tuple ::<'static,isize,2>(1,2).is_one());
	}
}


#[test]
fn custom_name(){
	mod inner{
		use super::*;

		#[derive(EnumIs)]
		pub enum CustomName{
			#[enum_is(name(pub custom))] Unit,
		}
	}

	assert!(inner::CustomName::Unit.custom());
}
