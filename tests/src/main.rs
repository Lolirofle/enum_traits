#![feature(plugin,custom_derive,custom_attribute)]
#![plugin(enumerated_macros)]
#![allow(non_camel_case_types)]

extern crate enumerated;

use enumerated::*;

#[derive(Copy,Clone,EnumToIndex)]
enum TestEnum<'t,T: 't>{
	VariantA(&'t T),
	VariantB(T),
	VariantC(T,T,T,T),
	VariantD{d: i32},
	VariantE{a: i32,b: i32,c: i32,d: i32,e: i32},
	VariantF,
}

fn main(){
	let i = 0u8;
	let mut e = TestEnum::VariantA(&i);
	println!("VariantA: {}",e.into_index());
	println!("VariantA: {}",e.index());

	e = TestEnum::VariantB(0);
	println!("VariantB: {}",e.into_index());
	println!("VariantB: {}",e.index());

	e = TestEnum::VariantC(0,1,2,3);
	println!("VariantC: {}",e.into_index());
	println!("VariantC: {}",e.index());

	e = TestEnum::VariantD{d: 0};
	println!("VariantD: {}",e.into_index());
	println!("VariantD: {}",e.index());

	e = TestEnum::VariantE{a: 0,b: 1,c: 2,d: 3,e: 4};
	println!("VariantE: {}",e.into_index());
	println!("VariantE: {}",e.index());

	e = TestEnum::VariantF;
	println!("VariantF: {}",e.into_index());
	println!("VariantF: {}",e.index());
}

//#[enum_as_separate_mod]
#[derive(EnumLen)]
pub enum TestEnum2{
	A{i: u32,i2: i32},
	B(u32,i32,u8),
	C
}

#[derive(EnumFromIndex,EnumToIndex,EnumLen,EnumEnds,EnumIterator)]
enum TestNoFields{
	A,B,C,D,E,F
}
