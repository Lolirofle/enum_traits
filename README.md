# enum_traits #

A library with traits and accompanying procedural macros that adds functionality to enums.

Provides traits and "derives" for enum items in the Rust programming language:

### Derives ###
- EnumIndex (impl Index)
- EnumFromIndex (impl FromIndex)
- EnumToIndex (impl ToIndex)
- EnumLen (impl Len)
- EnumEnds (impl Ends)
- EnumDiscriminant (impl Discriminant)
- EnumIter (impl Iterable)
- EnumIterator (impl Iterator)
- EnumVariantName (impl VariantName)
- EnumBitPattern (impl BitPattern)
- EnumUnitVariant (impl UnitVariant)
- EnumIsVariantFns
- EnumFromVariantName (impl FromStr)

### Traits ###
- Index
- FromIndex
- ToIndex
- Len
- Ends
- Discriminant
- Iterable
- VariantName
- BitPattern
- UnitVariant

### Usage ###

Cargo.toml:
```TOML
[dependencies]
enum_traits        = "*"
enum_traits_macros = "*"
```

With no_std:
```TOML
[dependencies]
enum_traits        = {version="*",features=["no_std"]}
enum_traits_macros = {version="*",features=["no_std"]}
```

### Examples ###
```rust
//#![feature(associated_consts)]

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
	#[derive(Debug,EnumIndex,EnumFromIndex,EnumToIndex,EnumLen,EnumIter,EnumIterator,EnumDiscriminant,EnumEnds)]
	enum Enum{
		VariantA = 10,
		VariantB = 20,
		VariantC = 30,
	}

	//From EnumToIndex
	assert_eq!(Enum::VariantB.into_index(),1);

	//From EnumLen
	//assert_eq!(Enum::LEN,3);
	assert_eq!(Enum::len(),3);

	//From EnumFromIndex
	assert!(match Enum::from_index(1){
		Some(Enum::VariantB) => true,
		_ => false
	});

	//From EnumDiscriminant
	assert!(match Enum::from_discriminant(20){
		Some(Enum::VariantB) => true,
		_ => false
	});

	//From EnumEnds
	assert!(match Enum::first(){
		Enum::VariantA => true,
		_ => false
	});

	//From EnumEnds
	assert!(match <Enum as Ends>::last(){
		Enum::VariantC => true,
		_ => false
	});

	//From EnumIter
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
```

See the tests for more examples.
See the [docs for the library](https://docs.rs/crate/enum_traits/), [docs for the derives](https://docs.rs/crate/enum_traits_macros/), the tests or the source code for more information.
