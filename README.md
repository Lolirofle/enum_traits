# enum_traits #

Traits and accompanying "derives" (with attributes as an alternative) on enums in the Rust programming language.

These crates contain procedural macros that add functionality to, extract additional information from, and generate boilerplate patterns for enums.

### Derives ###

The following derives in `enum_traits_macros` implements traits, both from the standard libraries and from `enum_traits`:

- EnumLen (impl enum_traits::EnumLen)
- EnumEnds (impl enum_traits::EnumEnds)
- EnumToIndex (impl enum_traits::EnumToIndex)
- EnumFromIndex (impl enum_traits::EnumFromIndex)
- EnumIndex (impl enum_traits::EnumIndex)
- EnumIterable (impl enum_traits::EnumIterable)
- EnumIterator (impl core::iter::Iterator)
- EnumVariantName (impl enum_traits::EnumVariantName)
- EnumFromVariantName (impl core::convert::FromStr)
- EnumTag (impl enum_traits::EnumTag)
- EnumIs
- EnumFrom (impl core::convert::From)
- EnumStep (impl enum_traits::EnumStep)
- EnumFromDiscriminant (impl enum_traits::EnumFromDiscriminant)
- EnumVariantsArray (impl enum_traits::EnumVariantsArray)
- EnumFieldStruct
- EnumInto (impl core::convert::Into)
- EnumTryInto (impl core::convert::TryInto)

### Traits ###

The following traits in `enum_traits` can be automatically derived using the "derives" above:

- EnumIndex
- EnumFromIndex
- EnumToIndex
- EnumLen
- EnumEnds
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
- impl_enum_to_index
- impl_enum_from_index
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

All derives/attributes can be toggled using features. See `macros/Cargo.toml` for a list of all of them.

### Examples ###
```rust
#[macro_use]extern crate enum_traits_macros;
extern crate enum_traits;

use enum_traits::*;

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

fn f2(){
	#[derive(Debug,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIterable,EnumIterator,EnumFromDiscriminant,EnumEnds)]
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

	//From EnumFromDiscriminant
	assert!(match Enum::from_discriminant(20){
		Some(Enum::VariantB) => true,
		_ => false
	});

	//From EnumEnds
	assert!(match Enum::FIRST{
		Enum::VariantA => true,
		_ => false
	});

	//From EnumEnds
	assert!(match <Enum as EnumEnds>::LAST{
		Enum::VariantC => true,
		_ => false
	});

	//From EnumIterable
	assert!(match Enum::into_iter().next(){
		Some(Enum::VariantA) => true,
		_ => false
	});

	//From EnumIterator
	assert!(match Enum::VariantA.next(){
		Some(Enum::VariantB) => true,
		_ => false
	});
}

fn f3(){
	//Using attributes (without any traits and derives)
	#[impl_enum_len(OUR_CUSTOM_LENGTH)]
	#[impl_enum_first(THIS_IS_THE_FIRST)]
	#[impl_enum_last(HERE_IS_THE_LAST)]
	#[impl_enum_variants_array(#[deprecated] pub A_LIST)]
	enum Enum{
		A,
		B,
		C,
		D,
	}

	assert_eq!(Enum::A,Enum::THIS_IS_THE_FIRST);
	assert_eq!(Enum::D,Enum::HERE_IS_THE_LAST);
	assert_eq!(Enum::OUR_CUSTOM_LENGTH,4);
	assert_eq!(Enum::OUR_CUSTOM_LENGTH,4);
	assert_eq!(Enum::A_LIST,[Enum::A,Enum::B,Enum::C,Enum::D]);
}
```

See the tests for more examples.
See the [docs for the library](https://docs.rs/crate/enum_traits/), [docs for the derives/attributes](https://docs.rs/crate/enum_traits_macros/), the tests or the source code for more information and additional examples.
