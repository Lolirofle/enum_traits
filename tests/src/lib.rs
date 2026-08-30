#![allow(dead_code)]
#![no_std]

mod attr_ends;
mod attr_field_structs;
mod attr_len;
mod derive_from;
mod derive_into;
mod derive_is;

#[macro_export] macro_rules! gen_test_attr_const{
	($e: expr,$name: tt ($($pre: tt)?) ($($post: tt)?) $(, #[ $($derives: tt)* ])?) => {
		#[test] fn no_prefix(){
			$(#[ $($derives)* ])? #[$name($($pre)* NAME $($post)*)] enum X{A,B,C}
			assert_eq!(X::NAME,$e);
		}

		#[test] fn const_prefix(){
			$(#[ $($derives)* ])? #[$name($($pre)* const NAME $($post)*)] enum X{A,B,C}
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_crate_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(crate) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_vis_const(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub const NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_super_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(super) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn pub_in_path_vis(){
			mod inner{use super::*; $(#[ $($derives)* ])? #[$name($($pre)* pub(in super) NAME $($post)*)] pub enum X{A,B,C}}
			use inner::*;
			assert_eq!(X::NAME,$e);
		}

		#[test] fn attr_conditional(){
			$(#[ $($derives)* ])? #[$name(#[cfg(all(test,not(test)))] $($pre)* NAME $($post)*)] enum X{A,B,C}
			impl X{const NAME: u64 = 0x85ce938eaf004a38;}
			assert_eq!(X::NAME,0x85ce938eaf004a38);
		}
	};
}

//Useful commands for testing:
//  cargo rustc -- -Z unstable-options --pretty=expanded --test
//  cargo expand-macros
//  cargo expand --test main
//  cargo expand --ugly --all-features > [FILE]
//  cargo -v rustc --release -- --emit=llvm-ir

use enum_traits_macros::*;
