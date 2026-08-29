#![allow(unused)]

use enum_traits_macros::*;

#[test]
fn single(){
	#[derive(EnumInto)]
	enum X{
		A(u8,u16,u32),
	}
	assert_eq!(1u8, X::A(1,2,3).into());
	assert_eq!(2u16,X::A(1,2,3).into());
	assert_eq!(3u32,X::A(1,2,3).into());
}

#[test]
fn common(){
	#[derive(EnumInto)]
	enum X{
		A(u8,u16,u32),
		B(u16,u32,u8),
		C(u32,u16,u8),
	}
	assert_eq!(1u8, X::A(1,2,3).into());
	assert_eq!(2u16,X::A(1,2,3).into());
	assert_eq!(6u8, X::B(4,5,6).into());
	assert_eq!(4u16,X::B(4,5,6).into());
	assert_eq!(9u8, X::C(7,8,9).into());
	assert_eq!(8u16,X::C(7,8,9).into());
}

#[test]
fn mix(){
	#[derive(EnumInto)]
	enum X{
		A(u8,i32),
		B(u8,f64,i32),
		C{a: u8,b: i32,c: bool},
	}
	assert_eq!(7u8   ,X::A(7,100).into());
	assert_eq!(100i32,X::A(7,100).into());
	assert_eq!(8u8   ,X::B(8,0.0,200).into());
	assert_eq!(200i32,X::B(8,0.0,200).into());
	assert_eq!(9u8   ,X::C{a: 9,b: 300,c: true}.into());
	assert_eq!(300i32,X::C{a: 9,b: 300,c: true}.into());
}

#[test]
fn most_frequent_name(){
	//`u8` appears under `y` in all record variants => `y` is chosen.
	#[derive(EnumInto)]
	enum X{
		A{x: u8,y: u8},
		B{y: u8,w: u8,h: u8},
		C{v: u32,y: u8},
	}
	assert_eq!(1u8,X::A{x: 100,y: 1}.into());
	assert_eq!(2u8,X::B{y: 2,w: 100,h: 55}.into());
	assert_eq!(3u8,X::C{y: 3,v: 100}.into());
}

#[test]
fn most_frequent_name_different_type(){
	#[derive(EnumInto)]
	enum X{
		A{v: u8},
		B{w: u8,v: u16},
		C{w: u8,v: u16,u: u32},
	}
	assert_eq!(1u8,X::A{v: 1}.into());
	assert_eq!(2u8,X::B{w: 2,v: 0}.into());
	assert_eq!(3u8,X::C{w: 3,v: 0,u: 0}.into());
}


#[test]
fn tuple_record_mix(){
	#[derive(EnumInto)]
	enum X{
		T(u8,u16),
		R{x: u8,y: u16},
		S(u8),
	}
	assert_eq!(1u8,X::T(1,2).into());
	assert_eq!(3u8,X::R{x: 3,y: 4}.into());
	assert_eq!(5u8,X::S(5).into());
}

#[test]
fn multiple_mix_fields(){
	#[derive(EnumInto)]
	enum X2{
		T1(u8,u16,u32),
		T2(u32,u16,u8),
		R{a: u8,b: u16,c: u32},
	}
	assert_eq!(1u8, X2::T1(1,2,3).into());
	assert_eq!(2u16,X2::T1(1,2,3).into());
	assert_eq!(3u32,X2::T1(1,2,3).into());
	assert_eq!(4u32,X2::T2(4,5,6).into());
	assert_eq!(5u16,X2::T2(4,5,6).into());
	assert_eq!(6u8 ,X2::T2(4,5,6).into());
	assert_eq!(7u8, X2::R{a: 7,b: 8,c: 9}.into());
	assert_eq!(8u16,X2::R{a: 7,b: 8,c: 9}.into());
	assert_eq!(9u32,X2::R{a: 7,b: 8,c: 9}.into());
}

#[test]
fn generic_type(){
	#[derive(EnumInto)]
	enum X<T>{
		A(Option<T>,u8),
		B(u8,Option<T>),
		C{t: Option<T>,n: u8},
	}
	assert_eq!(5u8,X::<&'static str>::A(Some("hi").into(),5).into());
	assert_eq!(6u8,X::<&'static str>::B(6,Some("yo").into()).into());
	assert_eq!(7u8,X::<&'static str>::C{t: Some("z").into(),n: 7}.into());
}

#[test]
fn where_bounds(){
	#[derive(EnumInto)]
	enum X<T> where
		T: Clone,
	{
		A(u8,Option<T>),
		B(Option<T>,u8),
	}
	assert_eq!(1u8,X::<String>::A(1,Some(String::new())).into());
	assert_eq!(2u8,X::<String>::B(Some(String::new()),2).into());
}

#[test]
fn lifetimes(){
	#[derive(EnumInto)]
	enum X<'a>{
		A(u8,&'a str),
		B(&'a str,u8),
		C{n: u8,s: &'a str},
	}
	let s = "x";
	assert_eq!(1u8,X::A(1,s).into());
	assert_eq!(2u8,X::B(s,2).into());
	assert_eq!(3u8,X::C{n: 3,s}.into());
}

#[test]
fn const_generics(){
	#[derive(EnumInto)]
	enum X<const N: usize>{
		A(u8,[u16; N]),
		B([u16; N],u8),
	}
	assert_eq!(1u8,X::<2>::A(1,[0,0]).into());
	assert_eq!(2u8,X::<2>::B([0,0],2).into());
}

#[test]
fn phantom_data(){
	use core::marker::PhantomData;
	#[derive(EnumInto)]
	enum X<T>{
		A(u8,PhantomData<T>),
		B(PhantomData<T>,u8),
	}
	assert_eq!(1u8,X::<u32>::A(1,PhantomData).into());
	assert_eq!(2u8,X::<u32>::B(PhantomData,2).into());
}

#[test]
fn reference(){
	#[derive(EnumInto)]
	enum X<'a>{
		A(&'a u8,u16),
		B(u16,&'a u8),
	}
	let x: &'static u8 = &7u8;
	let y: &'static u8 = &8u8;
	assert_eq!(x,<X<'_> as Into<&'static u8>>::into(X::A(x,0)));
	assert_eq!(y,<X<'_> as Into<&'static u8>>::into(X::B(0,y)));
}

#[test]
fn tuples(){
	#[derive(EnumInto)]
	enum X{
		A((u8,u16),u32),
		B(u32,(u8,u16)),
	}
	let t: (u8,u16) = X::A((1,2),0).into();
	assert_eq!(t,(1,2));
	let n: u32 = X::A((0,0),3).into();
	assert_eq!(n,3);
	let t: (u8,u16) = X::B(0,(4,5)).into();
	assert_eq!(t,(4,5));
}

#[test]
fn force_choice_by_alias(){
	type Alias = u8;

	#[derive(EnumInto)]
	enum E {
		A(u8,u16,Alias),
		B(Alias,u8),
		C(u8),
	}
	assert_eq!(1u8,E::A(1,2,3).into());
	assert_eq!(5u8,E::B(4,5).into());
	assert_eq!(6u8,E::C(6).into());
}
