//Useful commands for testing:
//  cargo rustc -- -Z unstable-options --pretty=expanded --test
//  cargo expand-macros
//  cargo expand --test main
//  cargo expand --ugly --all-features > [FILE]
//  cargo -v rustc --release -- --emit=llvm-ir

#![no_std]

/*
use enum_traits_macros::*;

#[derive(EnumFieldStructs)]
enum Fields{
	VariantA(i8),
	VariantB(i32),
	VariantC(u8,u16,u32),
	VariantD{d: (u8,i32)},
	VariantE{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
	VariantF,
}
*/
