# enum_traits #

A library with traits and accompanying procedural macros that adds functionality to enums.

Provides traits, "derives" and attributes for enum items in the Rust programming language:

### Derives ###
- EnumIndex (impl Index)
- EnumFromIndex (impl FromIndex)
- EnumToIndex (impl ToIndex)
- EnumLen (impl Len)
- EnumEnds (impl Ends)
- EnumDiscriminant (impl Discriminant)
- EnumIterable (impl Iterable)
- EnumIterator (impl Iterator)
- EnumVariantName (impl VariantName)
- EnumIs
- EnumFromStr (impl FromStr)
- EnumTag
- EnumFrom (impl From)
- EnumStep (impl Step)
- EnumFromDiscriminant (impl FromDiscriminant)

### Traits ###
- Index
- FromIndex
- ToIndex
- Len
- Ends
- VariantName
- Tag
- Step
- IntoDiscriminant
- FromDiscriminant

### Attributes ###
- impl_enum_len
- impl_enum_first
- impl_enum_last
- impl_enum_variants_array

### Usage ###

Cargo.toml:
```TOML
[dependencies]
enum_traits        = "*"
enum_traits_macros = "*"
```

All derives can be toggled using features.

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
	assert!(match <Enum as Ends>::LAST{
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
See the [docs for the library](https://docs.rs/crate/enum_traits/), [docs for the derives](https://docs.rs/crate/enum_traits_macros/), the tests or the source code for more information and additional examples.
