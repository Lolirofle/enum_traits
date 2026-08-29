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
fn index(){
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
	let _: <Fields<i32> as EnumIndex>::Type = e.index();
}

#[test]
fn len(){
	assert_eq!(6,Fields::<'static,u32>::LEN);
}

#[test]
fn isvariantfns(){
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
fn tag(){
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
fn variant_name(){
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
