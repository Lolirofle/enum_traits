# enum_traits #

Traits and accompanying "derives" (with attributes as an alternative) on enums in the Rust programming language.

These crates contain procedural macros that add functionality to, extract additional information from, and generate boilerplate patterns for enums.

### Derives ###

The following derives in `enum_traits_macros` implements traits, both from the standard libraries and from `enum_traits`:

- EnumLen (impl enum_traits::EnumLen)
- EnumEnds (impl enum_traits::EnumEnds)
- EnumIndex (impl enum_traits::EnumIndex)
- EnumFromIndex (impl enum_traits::EnumFromIndex)
- EnumToIndex (impl enum_traits::EnumToIndex)
- EnumIterable (impl enum_traits::EnumIterable)
- EnumIterator (impl core::iter::Iterator)
- EnumVariantName (impl enum_traits::EnumVariantName)
- EnumFromVariantName (impl core::convert::FromStr)
- EnumTag (impl enum_traits::EnumTag)
- EnumStep (impl enum_traits::EnumStep)
- EnumFromDiscriminant (impl enum_traits::EnumFromDiscriminant)
- EnumVariantsArray (impl enum_traits::EnumVariantsArray)
- EnumFrom (impl core::convert::From)
- EnumInto (impl core::convert::Into)
- EnumTryInto (impl core::convert::TryInto)
- EnumIs
- EnumFieldStruct

### Traits ###

The following traits in `enum_traits` can be automatically derived using the "derives" above:

- EnumLen
- EnumEnds
- EnumIndex
- EnumFromIndex
- EnumToIndex
- EnumIterable
- EnumVariantName
- EnumTag
- EnumStep
- EnumIntoDiscriminant
- EnumFromDiscriminant
- EnumVariantsArray

### Attributes ###

The attributes are related to the "derives" in the sense that they provide similar functionalities, but often without using any traits at all (no dependency on `enum_traits`).

The following attributes in `enum_traits_macros` with prefix `impl_*` implements items (functions, constants) on the enum item:

- impl_enum_len
- impl_enum_first
- impl_enum_last
- impl_enum_from_index
- impl_enum_to_index
- impl_enum_variant_name
- impl_enum_from_variant_name
- impl_enum_from_variant_name_default
- impl_enum_prev
- impl_enum_next
- impl_enum_variants_array
- impl_enum_into

The following attributes creates new items (structs, enums) based on the enum item:

- enum_tag

The following attributes transforms the enum item in some way:

- transform_enum_field_struct

### Usage ###

Cargo.toml:
```TOML
[dependencies]
enum_traits        = "*"
enum_traits_macros = "*"
```

All derives/attributes can be toggled using features. See `macros/Cargo.toml` for a list of all of them and the [API documentation](https://docs.rs/crate/enum_traits_macros/) for what functionality they toggle. All features are by default enabled.

Example of only enabling the `derives` features in Cargo.toml:
```TOML
[dependencies]
enum_traits = *
enum_traits_macros = {version = "*", default-features = false, features = ["derives"]}
```

### Example code ###
```rust
#[macro_use] extern crate enum_traits_macros;
extern crate enum_traits;

use enum_traits::*;
use enum_traits_macros::*;

fn derives_with_fields(){
	#[derive(Debug,EnumLen,EnumIndex,EnumToIndex,EnumVariantName,EnumTag,EnumFrom,EnumIs)]
	enum Fields<'t,T: 't>{
		VariantA(&'t T),
		VariantB(T,T),
		VariantC(T,T,T),
		VariantD{d: i32},
		VariantE{a: i8,b: i16,c: i32},
		VariantF,
	}

	assert_matches!(Fields::<'static,&'static str>::LEN,6);
	assert_matches!(Fields::VariantB("OK","KO").into_index(),1);
	assert_matches!(Fields::VariantC('X','Y','Z').variant_name(),"VariantC");
	assert_matches!(Fields::<char>::VariantD{d: 2}.tag(),FieldsTag::VariantD);
	assert_matches!(Fields::<char>::from((1i8,2i16,3i32)),Fields::VariantE{a: 1,b: 2,c: 3});
	assert!(Fields::<char>::VariantF.is_variant_f());
}

fn derives_with_discriminants(){
	#[derive(Debug,EnumLen,EnumEnds,EnumIndex,EnumFromIndex,EnumToIndex,EnumIterable,EnumIterator,EnumVariantName,EnumFromVariantName,EnumStep,EnumFromDiscriminant,EnumVariantsArray,EnumIs)]
	enum Discr{
		VariantA = 10,
		VariantB = 20,
		VariantC = 30,
	}
	impl_IntoDiscriminant_of_numeric!(u8,Discr);

	assert_eq!(Discr::LEN,3);
	assert_matches!(Discr::FIRST , Discr::VariantA);
	assert_matches!(Discr::LAST ,  Discr::VariantC);
	assert_matches!(Discr::from_index(1) , Some(Discr::VariantB));
	assert_eq!(Discr::VariantB.into_index(),1);
	assert_eq!(Discr::VariantC.index(),2);
	assert_matches!(Discr::variants().next() , Some(Discr::VariantA));
	assert_matches!(Discr::VariantA.next() , Some(Discr::VariantB));
	assert_matches!(Discr::VariantA.variant_name() , "VariantA");
	assert_matches!(Discr::VariantB.previous() , Some(Discr::VariantA));
	assert_matches!(Discr::VariantB.next()     , Some(Discr::VariantC));
	assert_matches!(Discr::from_discriminant(20) , Some(Discr::VariantB));
	assert_matches!(Discr::VARIANTS , [Discr::VariantA,Discr::VariantB,Discr::VariantC]);
	assert!(Discr::VariantB.is_variant_b());
}

fn derives_with_types_in_common(){
	#[derive(EnumInto)]
	enum Common1{
		VariantA(u8,u16),
		VariantB{x: u8,y: i16},
		VariantC(i16,u8,u8),
	}

	assert_eq!(1u8,Common1::VariantA(1,2).into());
	assert_eq!(3u8,Common1::VariantB{x: 3,y: 4}.into());
	assert_eq!(6u8,Common1::VariantC(5,6,7).into());

	#[derive(EnumTryInto)]
	enum Common2{
		VariantA(u8,u16),
		VariantB{x: u8,y: i16},
		VariantC(i16,u8,u8),
	}
	assert_eq!(Ok(1u8) ,Common2::VariantA(1,2).try_into());
	assert_eq!(Ok(2u16),Common2::VariantA(1,2).try_into());
	assert_eq!(Ok(3u8) ,Common2::VariantB{x: 3,y: 4}.try_into());
	assert_eq!(Ok(4i16),Common2::VariantB{x: 3,y: 4}.try_into());
	assert_eq!(Ok(6u8) ,Common2::VariantC(5,6,7).try_into());
}

fn attributes(){ //Without any traits and derives
	#[impl_enum_len(OUR_LENGTH)]
	#[impl_enum_first(THIS_IS_THE_FIRST)]
	#[impl_enum_last(HERE_IS_THE_LAST)]
	#[impl_enum_from_index(const fn from_a_number(_: u8) -> Option<_>)]
	#[impl_enum_to_index(const fn to_a_number(&self) -> u16)]
	#[impl_enum_variant_name(const fn name(&self) -> &str)]
	#[impl_enum_from_variant_name(fn from_name)]
	#[impl_enum_prev(fn go_back(self) -> Option<Self>)]
	#[impl_enum_next(fn go_forward(self) -> Option<Self>)]
	#[impl_enum_variants_array(#[deprecated] pub THE_LIST)]
	#[derive(Debug)]
	enum Units{
		A,
		B,
		C,
		D,
	}

	assert_matches!(Units::OUR_LENGTH,4);
	assert_matches!(Units::THIS_IS_THE_FIRST,Units::A);
	assert_matches!(Units::HERE_IS_THE_LAST ,Units::D);
	assert_matches!(Units::from_a_number(2),Some(Units::C));
	assert_matches!(Units::C.to_a_number(),2);
	assert_matches!(Units::C.name(),"C");
	assert_matches!(Units::from_name("C"),Some(Units::C));
	assert_matches!(Units::C.go_back(),Some(Units::B));
	assert_matches!(Units::C.go_forward(),Some(Units::D));
	assert_matches!(Units::THE_LIST,[Units::A,Units::B,Units::C,Units::D]);

	#[impl_enum_into(
		ty(u8  => fn to_u8(&self) -> &_),
		ty(u32 => fn to_u32(self) -> Option<_>)
	)]
	enum Common{
		A(u8,u16),
		B{x: u8,y: u32},
		C(u32,u8),
	}
}
```

See the tests for more examples.

See the [API docs for the library](https://docs.rs/crate/enum_traits/), [docs for the derives/attributes](https://docs.rs/crate/enum_traits_macros/), the tests or the source code for more information and additional examples.
