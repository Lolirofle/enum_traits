#![allow(non_snake_case)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use std::{cmp,iter};
use std::ascii::AsciiExt;
use std::iter::FromIterator;
use syn::{Attribute,Body,Expr,ExprKind,Ident,Lit,IntTy,MacroInput,Variant,VariantData};
use quote::Tokens;

fn minimum_type_from_value(value: usize) -> Ident{
	if value <= u8::max_value() as usize{
		Ident::from("u8")
	}else if value <= u16::max_value() as usize{
		Ident::from("u16")
	}else if value <= u32::max_value() as usize{
		Ident::from("u32")
	}else if value <= u64::max_value() as usize{
		Ident::from("u64")
	}else{
		Ident::from("usize")
	}
}

fn type_from_repr_attr<'i,I>(attrs: I) -> Option<Ident>
	where I: Iterator<Item = &'i Attribute>
{
	use syn::{MetaItem,NestedMetaItem};

	for attr in attrs{match attr.value{
		MetaItem::List(ref ident,ref content) if ident=="repr" => match &content[0]{
			&NestedMetaItem::MetaItem(MetaItem::Word(ref ty)) if ty!="C" => return Some(ty.clone()),
			_ => continue,
		},
		_ => continue,
	}}
	None
}

fn variant_unit_ident<'v>(variant: &'v Variant,derive_name: &'static str) -> &'v Ident{match variant.data{
	VariantData::Unit => {
		&variant.ident
	}
	_ => panic!(format!("`derive({})` may only be applied to enum items with no fields",derive_name))
}}

fn derive_enum<F>(input: TokenStream,gen_impl: F) -> TokenStream
	where F: FnOnce(&Ident,&MacroInput,&Vec<Variant>) -> Tokens
{
	let input = input.to_string();
	let ast = syn::parse_macro_input(&input).unwrap();

	let quote_tokens = match ast.body{
		Body::Enum(ref data) => gen_impl(&ast.ident,&ast,data),
		_ => panic!("`derive(Enum*)` may only be applied to enum items")
	}.to_string();

	format!("{}",quote_tokens.to_string()).parse().unwrap()
}

#[allow(dead_code)]
fn minimum_type_containing_enum(item: &MacroInput,data: &Vec<Variant>) -> syn::Ident{//TODO: Maybe useful to export?
	//First, check if there's a repr attribute
	type_from_repr_attr(item.attrs.iter())
	.unwrap_or_else(||
		//Second, use the maximum value of an explicit discriminant or the length of the enum (depending on which is the greatest)
		minimum_type_from_value(match data.iter().filter_map(|variant| match variant.discriminant{
				Some(syn::ConstExpr::Lit(syn::Lit::Int(discrimimant,_))) => Some(discrimimant),
				_ => None
			}).max(){
				Some(max) => cmp::max(cmp::max(data.len(),1)-1 , max as usize),
				//Third, use the length of the enum
				_ => cmp::max(data.len(),1)-1
			}
		)
	)
}

/// Implements `enum_traits::Len`, a constant that indicates the number of variants of an enum.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// {
/// 	#[derive(EnumLen)]enum T{}
/// 	assert_eq!(0,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A}
/// 	assert_eq!(1,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C}
/// 	assert_eq!(3,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G}
/// 	assert_eq!(7,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H}
/// 	assert_eq!(8,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
/// 	assert_eq!(25,T::LEN);
/// }{
/// 	#[derive(EnumLen)]enum T{A,B(),C{},D(u8),E{e: u8},F(u8,u16),G{g1: u8,g2: u16},H}
/// 	assert_eq!(8,T::LEN);
/// }
/// # }
/// ```
#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: TokenStream) -> TokenStream{ //TODO: Consider allowing structs. Number of variants of struct is always 1
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let len = data.len();

		#[cfg(feature = "stable")]
		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Len for #ident #ty_generics #where_clause{
				fn len() -> usize{#len}
			}
		}

		#[cfg(not(feature = "stable"))]
		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Len for #ident #ty_generics #where_clause{
				const LEN: usize = #len;
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::Ends`, two constructors that constructs the first and the last variant of an enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum has at least one variant
/// - The enum's first variant is an unit variant
/// - The enum's last variant is an unit variant
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// {
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::A,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::B,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::C,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::G,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::H,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::Z,T::last());
/// }{
/// 	#[derive(Debug,Eq,PartialEq,EnumEnds)]enum T{A,B(),C{},D(u8),E{e: u8},F(u8,u16),G{g1: u8,g2: u16},H}
/// 	assert_eq!(T::A,T::first());
/// 	assert_eq!(T::H,T::last());
/// }
/// # }
/// ```
#[proc_macro_derive(EnumEnds)]
pub fn derive_EnumEnds(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let variant_first_ident = &data.first().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;
		let variant_last_ident  = &data.last().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Ends for #ident #ty_generics #where_clause{
				#[inline(always)]fn first() -> Self{#ident::#variant_first_ident}
				#[inline(always)]fn last()  -> Self{#ident::#variant_last_ident}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::ToIndex`, a function that returns the index of a variant of an enum in the defined order.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// {
/// 	//TODO: Fix this: #[derive(EnumIndex,EnumToIndex)]enum T{}
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]enum T{A}
/// 	assert_eq!(0,T::A.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]enum T{A,B,C,D,E,F,G,H}
/// 	assert_eq!(0,T::A.index());
/// 	assert_eq!(1,T::B.index());
/// 	assert_eq!(2,T::C.index());
/// 	assert_eq!(3,T::D.index());
/// 	assert_eq!(4,T::E.index());
/// 	assert_eq!(5,T::F.index());
/// 	assert_eq!(6,T::G.index());
/// 	assert_eq!(7,T::H.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// 	assert_eq!(1,T::B.into_index());
/// 	assert_eq!(2,T::C.into_index());
/// 	assert_eq!(3,T::D.into_index());
/// 	assert_eq!(4,T::E.into_index());
/// 	assert_eq!(5,T::F.into_index());
/// 	assert_eq!(6,T::G.into_index());
/// 	assert_eq!(7,T::H.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
/// 	assert_eq!(00,T::A.index());
/// 	assert_eq!(01,T::B.index());
/// 	assert_eq!(02,T::C.index());
/// 	assert_eq!(03,T::D.index());
/// 	assert_eq!(04,T::E.index());
/// 	assert_eq!(05,T::F.index());
/// 	assert_eq!(06,T::G.index());
/// 	assert_eq!(07,T::H.index());
/// 	assert_eq!(08,T::I.index());
/// 	assert_eq!(09,T::J.index());
/// 	assert_eq!(10,T::K.index());
/// 	assert_eq!(11,T::L.index());
/// 	assert_eq!(12,T::M.index());
/// 	assert_eq!(13,T::N.index());
/// 	assert_eq!(14,T::O.index());
/// 	assert_eq!(15,T::P.index());
/// 	assert_eq!(16,T::Q.index());
/// 	assert_eq!(17,T::R.index());
/// 	assert_eq!(18,T::S.index());
/// 	assert_eq!(19,T::T.index());
/// 	assert_eq!(20,T::U.index());
/// 	assert_eq!(21,T::V.index());
/// 	assert_eq!(22,T::X.index());
/// 	assert_eq!(23,T::Y.index());
/// 	assert_eq!(24,T::Z.index());
///
/// 	assert_eq!(00,T::A.into_index());
/// 	assert_eq!(01,T::B.into_index());
/// 	assert_eq!(02,T::C.into_index());
/// 	assert_eq!(03,T::D.into_index());
/// 	assert_eq!(04,T::E.into_index());
/// 	assert_eq!(05,T::F.into_index());
/// 	assert_eq!(06,T::G.into_index());
/// 	assert_eq!(07,T::H.into_index());
/// 	assert_eq!(08,T::I.into_index());
/// 	assert_eq!(09,T::J.into_index());
/// 	assert_eq!(10,T::K.into_index());
/// 	assert_eq!(11,T::L.into_index());
/// 	assert_eq!(12,T::M.into_index());
/// 	assert_eq!(13,T::N.into_index());
/// 	assert_eq!(14,T::O.into_index());
/// 	assert_eq!(15,T::P.into_index());
/// 	assert_eq!(16,T::Q.into_index());
/// 	assert_eq!(17,T::R.into_index());
/// 	assert_eq!(18,T::S.into_index());
/// 	assert_eq!(19,T::T.into_index());
/// 	assert_eq!(20,T::U.into_index());
/// 	assert_eq!(21,T::V.into_index());
/// 	assert_eq!(22,T::X.into_index());
/// 	assert_eq!(23,T::Y.into_index());
/// 	assert_eq!(24,T::Z.into_index());
/// }{
/// 	#[derive(EnumIndex,EnumToIndex)]enum T{A,B(),C{},D(u8),E{e: u8},F(u8,u16),G{g1: u8,g2: u16},H}
/// 	assert_eq!(0,T::A.index());
/// 	assert_eq!(1,T::B().index());
/// 	assert_eq!(2,T::C{}.index());
/// 	assert_eq!(3,T::D(0).index());
/// 	assert_eq!(4,T::E{e: 0}.index());
/// 	assert_eq!(5,T::F(0,0).index());
/// 	assert_eq!(6,T::G{g1: 0,g2: 0}.index());
/// 	assert_eq!(7,T::H.index());
///
/// 	assert_eq!(0,T::A.into_index());
/// 	assert_eq!(1,T::B().into_index());
/// 	assert_eq!(2,T::C{}.into_index());
/// 	assert_eq!(3,T::D(0).into_index());
/// 	assert_eq!(4,T::E{e: 0}.into_index());
/// 	assert_eq!(5,T::F(0,0).into_index());
/// 	assert_eq!(6,T::G{g1: 0,g2: 0}.into_index());
/// 	assert_eq!(7,T::H.into_index());
/// }
/// # }
/// ```
#[proc_macro_derive(EnumToIndex)]
pub fn derive_EnumToIndex(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		let match_arms = data.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);

			match variant.data{
				VariantData::Unit => {
					quote! { &#ident::#variant_ident => #i, }
				}
				VariantData::Tuple(_) => {
					quote! { &#ident::#variant_ident(..) => #i, }
				}
				VariantData::Struct(_) => {
					quote! { &#ident::#variant_ident{..} => #i, }
				}
			}
		});

		let match_arms_into = data.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);

			match variant.data{
				VariantData::Unit => {
					quote! { #ident::#variant_ident => #i, }
				}
				VariantData::Tuple(_) => {
					quote! { #ident::#variant_ident(..) => #i, }
				}
				VariantData::Struct(_) => {
					quote! { #ident::#variant_ident{..} => #i, }
				}
			}
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::ToIndex for #ident #ty_generics #where_clause{
				fn into_index(self) -> <Self as Index>::Type{
					match self{
						#( #match_arms_into )*
					}
				}
				fn index(&self) -> <Self as Index>::Type{
					match self{
						#( #match_arms )*
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::FromIndex`, a function that maybe returns a variant of an enum from an supposed index in the defined order.
///
/// # Requirements
/// - The derived item is an enum
#[proc_macro_derive(EnumFromIndex)]
pub fn derive_EnumFromIndex(input: TokenStream) -> TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumFromIndex")
	}

	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		fn match_arm_transform(ident: &Ident,(i,variant_ident): (usize,&Ident)) -> Tokens{
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);
			quote! { #i => #ident::#variant_ident, }
		}
		let match_arms1 = data.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));
		let match_arms2 = data.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::FromIndex for #ident #ty_generics #where_clause{
				#[inline]
				fn from_index(index: <Self as Index>::Type) -> Option<Self>{
					Some(match index{
						#( #match_arms1 )*
						_ => return None
					})
				}

				#[inline]
				unsafe fn from_index_unchecked(index: <Self as Index>::Type) -> Self{
					match index{
						#( #match_arms2 )*
						_ => ::std::mem::uninitialized()
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::Index`.
///
/// # Requirements
/// - The derived item is an enum
#[proc_macro_derive(EnumIndex)]
pub fn derive_EnumIndex(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		//Determine which type to use (attribute or number of variants)
		let ty = type_from_repr_attr(item.attrs.iter())
			.unwrap_or_else(|| minimum_type_from_value(cmp::max(data.len(),1)-1));

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Index for #ident #ty_generics #where_clause{
				type Type = #ty;
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Creates a struct and implements `enum_traits::Iterable`.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum variants is all unit variants
#[proc_macro_derive(EnumIter)]
pub fn derive_EnumIter(input: TokenStream) -> TokenStream{//TODO: Consider rewriting output (EnumIter may not need Option, but then empty enums are not represented. Are they necessary to include?)
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumIter")
	}

	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let visibility = &item.vis;

		let len = data.len();
		let last = data.last();

		let prev_match_arms = {
				let iter = data.iter().rev().map(variant_unit_ident);
				iter.zip(data.iter().rev().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
			});

		let next_match_arms = {
				let iter = data.iter().map(variant_unit_ident);
				iter.zip(data.iter().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
			});

		let len_match_arms = data.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { &#ident::#variant_ident => #i, }
		});

		let count_match_arms = data.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { #ident::#variant_ident => #i, }
		});

		let variant_first_ident = &data.first().expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;
		let variant_last_ident  = &data.last().expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;

		let struct_ident = {
			let mut str = ident.as_ref().to_owned();
			str.push_str("Iter");
			Ident::from(str)
		};

		let struct_iter = quote!{
			#visibility struct #struct_ident #ty_generics #where_clause (pub Option<#ident #ty_generics>);
		};

		let impl_default = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::default::Default for #struct_ident #ty_generics #where_clause{
				#[inline(always)]
				fn default() -> Self{#struct_ident (None)}
			}
		};

		let impl_iter = {
			let fn_next = quote!{
				#[inline]
				fn next(&mut self) -> Option<Self::Item>{
					Some(match &self.0{
						&None => {self.0 = Some(#ident::#variant_first_ident); #ident::#variant_first_ident},
						#( #next_match_arms )*
						_ => return None
					})
				}
			};

			let fn_size_hint = quote!{
				#[inline(always)]
				fn size_hint(&self) -> (usize,Option<usize>){
					use ::std::iter::ExactSizeIterator;
					(self.len(),Some(self.len()))
				}
			};

			let fn_count = quote!{
				#[inline(always)]
				fn count(self) -> usize{
					self.0.map_or(#len,|variant|{
						#len - 1 - match variant{
							#( #count_match_arms )*
						}
					})
				}
			};

			let fn_last = if let Some(last_ident) = last.map(variant_unit_ident){quote!{
				#[inline(always)]
				fn last(self) -> Option<Self::Item>{
					Some(#ident::#last_ident)
				}
			}}else{quote!{
				#[inline(always)]
				fn last(self) -> Option<Self::Item>{
					None
				}
			}};

			quote!{
				#[automatically_derived]
				#[allow(unused_attributes)]
				impl #impl_generics ::std::iter::Iterator for #struct_ident #ty_generics #where_clause{
					type Item = #ident;

					#fn_next
					#fn_size_hint
					#fn_count
					#fn_last
				}
			}
		};

		//TODO: May be an incorrect use of DoubleEndedIterator. Use Step instead
		let impl_diter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::iter::DoubleEndedIterator for #struct_ident #ty_generics #where_clause{
				#[inline]
				fn next_back(&mut self) -> Option<Self::Item>{
					Some(match &self.0{
						&None => {self.0 = Some(#ident::#variant_last_ident); #ident::#variant_last_ident},
						#( #prev_match_arms )*
						_ => return None
					})
				}
			}
		};

		let impl_exactiter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::iter::ExactSizeIterator for #struct_ident #ty_generics #where_clause{
				#[inline]
				fn len(&self) -> usize{
					self.0.as_ref().map_or(#len,|variant|{
						#len - 1 - match variant{
							#( #len_match_arms )*
						}
					})
				}
			}
		};

		let impl_intoiter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Iterable for #ident #ty_generics #where_clause{
				type Iter = #struct_ident;
				#[inline(always)]fn variants() -> Self::Iter{#struct_ident(None)}
			}
		};

		quote!{
			#struct_iter
			#impl_intoiter
			#impl_default
			#impl_iter
			#impl_diter
			#impl_exactiter
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `Iterator`.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum variants is all unit variants
#[proc_macro_derive(EnumIterator)]
pub fn derive_EnumIterator(input: TokenStream) -> TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumIterator")
	}

	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		let len = data.len();
		let last = data.last();

		let prev_match_arms = {
				let iter = data.iter().rev().map(variant_unit_ident);
				iter.zip(data.iter().rev().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});

		let next_match_arms = {
				let iter = data.iter().map(variant_unit_ident);
				iter.zip(data.iter().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});

		let len_match_arms = data.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { &#ident::#variant_ident => #i, }
		});

		let count_match_arms = data.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(i as u64,IntTy::Unsuffixed);
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { #ident::#variant_ident => #i, }
		});

		let impl_iter = {
			let fn_next = quote!{
				#[inline]
				#[allow(unreachable_code)]
				fn next(&mut self) -> Option<Self::Item>{
					Some(match self{
						#( #next_match_arms )*
						_ => return None
					})
				}
			};

			let fn_size_hint = quote!{
				#[inline(always)]
				fn size_hint(&self) -> (usize,Option<usize>){
					use ::std::iter::ExactSizeIterator;
					(self.len(),Some(self.len()))
				}
			};

			let fn_count = quote!{
				#[inline]
				fn count(self) -> usize{
					#len - 1 - match self{
						#( #count_match_arms )*
					}
				}
			};

			let fn_last = if let Some(last_ident) = last.map(variant_unit_ident){quote!{
				#[inline(always)]
				fn last(self) -> Option<Self::Item>{
					Some(#ident::#last_ident)
				}
			}}else{quote!{
				#[inline(always)]
				fn last(self) -> Option<Self::Item>{
					None
				}
			}};

			quote!{
				#[automatically_derived]
				#[allow(unused_attributes)]
				impl #impl_generics ::std::iter::Iterator for #ident #ty_generics #where_clause{
					type Item = Self;
					#fn_next
					#fn_count
					#fn_size_hint
					#fn_last
				}
			}
		};

		/*let impl_diter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::iter::DoubleEndedIterator for #ident #ty_generics #where_clause{
				#[inline]
				#[allow(unreachable_code)]
				fn next_back(&mut self) -> Option<Self::Item>{
					Some(match self{
						#( #prev_match_arms )*
						_ => return None
					})
				}
			}
		};*/

		let impl_eiter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::iter::ExactSizeIterator for #ident #ty_generics #where_clause{
				#[inline]
				fn len(&self) -> usize{
					#len - 1 - match self{
						#( #len_match_arms )*
					}
				}
			}
		};

		quote!{
			#impl_iter
			//#impl_diter
			#impl_eiter
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::Discriminant`.
///
/// # Requirements
/// - The derived item is an enum
/// - The enum variants is all unit variants
#[proc_macro_derive(EnumDiscriminant)]
pub fn derive_EnumDiscriminant(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		fn variant_to_match_arm(ident: &Ident,variant: &Variant,only_unit_variants: bool) -> Option<Tokens>{
			let variant_ident = &variant.ident;

			//If an explicit discriminant exists
			variant.discriminant.as_ref().map(|ref variant_discriminant|{
				match variant.data{
					VariantData::Unit => {
						quote! { #variant_discriminant => #ident::#variant_ident, }
					}
					VariantData::Tuple(_)  |
					VariantData::Struct(_) => {
						//Tuple and struct variants cannot have explicit discriminants
						unreachable!()
					}
				}
			}).or_else(||{
				match variant.data{
					VariantData::Unit if only_unit_variants => Some({
						quote! { n if n==#ident::#variant_ident as Self::Type => #ident::#variant_ident, }
					}),
					_ => None
				}
			})
		};
		let only_unit_variants = data.iter().all(|variant| match variant.data{VariantData::Unit => true , _ => false});
		let match_arms1 = data.iter().filter_map(|variant| variant_to_match_arm(ident,variant,only_unit_variants));
		let match_arms2 = data.iter().filter_map(|variant| variant_to_match_arm(ident,variant,only_unit_variants));
		let ty = type_from_repr_attr(item.attrs.iter()).unwrap_or(Ident::from("usize"));

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Discriminant for #ident #ty_generics #where_clause{
				type Type = #ty;

				#[inline]
				fn from_discriminant(discriminant: <Self as Discriminant>::Type) -> Option<Self>{
					Some(match discriminant{
						#( #match_arms1 )*
						_ => return None
					})
				}

				#[inline]
				unsafe fn from_discriminant_unchecked(discriminant: <Self as Discriminant>::Type) -> Self{
					match discriminant{
						#( #match_arms2 )*
						_ => ::std::mem::uninitialized()
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements `enum_traits::EnumVariantName`, giving the name of the variants of an enum as a string.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// #[derive(EnumVariantName)]
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
/// assert_eq!(Enum::Dog.variant_name(), "Dog");
/// assert_eq!(Enum::Cat(0).variant_name(), "Cat");
/// assert_eq!(Enum::Robot{speed: 0.0}.variant_name(), "Robot");
/// # }
/// ```
#[proc_macro_derive(EnumVariantName)]
pub fn derive_EnumVariantName(input: TokenStream) -> TokenStream {
	fn gen_impl(ident: &Ident, item: &MacroInput, data: &Vec<Variant>) -> Tokens {
		let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

		let match_arms = data.iter().map(|variant| {
			let variant_ident = &variant.ident;
			let variant_str = variant.ident.as_ref();

			match variant.data {
				VariantData::Unit => {
					quote! { &#ident::#variant_ident => #variant_str, }
				}
				VariantData::Tuple(_) => {
					quote! { &#ident::#variant_ident(..) => #variant_str, }
				}
				VariantData::Struct(_) => {
					quote! { &#ident::#variant_ident{..} => #variant_str, }
				}
			}
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::VariantName for #ident #ty_generics #where_clause{
				#[inline]
				fn variant_name(&self) -> &'static str{
					match self{
						#( #match_arms )*
					}
				}
			}
		}
	}
	derive_enum(input, gen_impl)
}

#[proc_macro_derive(EnumFromVariantName)]
pub fn derive_EnumFromVariantName(input: TokenStream) -> TokenStream {//TODO: Consider not using FromStr, instead an own trait
	fn gen_impl(ident: &Ident, item: &MacroInput, data: &Vec<Variant>) -> Tokens {
		let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

		let match_arms = data.iter().filter_map(|variant| {
			let variant_ident = &variant.ident;
			let variant_str = variant.ident.as_ref();

			if let VariantData::Unit = variant.data{
				Some(quote! { #variant_str => #ident::#variant_ident, })
			}else{
				None
			}
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::std::str::FromStr for #ident #ty_generics #where_clause{
				type Err = ();

				fn from_str(str: &str) -> Result<Self,Self::Err>{
					Ok(match str{
						#( #match_arms )*
						_ => return Err(())
					})
				}
			}
		}
	}
	derive_enum(input, gen_impl)
}

/// Implements `enum_traits::BitPattern`.
///
/// # Requirements
/// - The derived item is an enum
#[proc_macro_derive(EnumBitPattern)]
pub fn derive_EnumBitPattern(input: TokenStream) -> TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumBitPattern")
	}

	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

		let n = (data.len() as f64 / 8.0).ceil() as usize;
		fn match_arm_transform(ident: &Ident,(i,variant_ident): (usize,&Ident),n: usize) -> Tokens{
			let lit = Expr::from(ExprKind::Array({
				let mut l = Vec::from_iter(iter::repeat(Expr::from(ExprKind::Lit(Lit::Int(0,IntTy::Unsuffixed)))).take(n));
				l[n-i/8-1] = Expr::from(ExprKind::Lit(Lit::Int((0b00000001u8.rotate_left((i as u32)%8) as u64),IntTy::Unsuffixed)));
				l
			}));
			quote! { #ident::#variant_ident => #lit, }
		}
		fn match_arm_transform_rev(ident: &Ident,(i,variant_ident): (usize,&Ident),n: usize) -> Tokens{
			let lit = Expr::from(ExprKind::Array({
				let mut l = Vec::from_iter(iter::repeat(Expr::from(ExprKind::Lit(Lit::Int(0,IntTy::Unsuffixed)))).take(n));
				l[i/8] = Expr::from(ExprKind::Lit(Lit::Int((0b10000000u8.rotate_right((i as u32)%8) as u64),IntTy::Unsuffixed)));
				l
			}));
			quote! { #ident::#variant_ident => #lit, }
		}
		let match_arms     = data.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg,n));
		let match_arms_rev = data.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform_rev(ident,arg,n));

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::BitPattern for #ident #ty_generics #where_clause{
				type ByteArray = [u8; #n];

				#[inline]
				fn bit_pattern(self) -> Self::ByteArray{
					match self{
						#( #match_arms )*
					}
				}

				#[inline]
				fn bit_pattern_rev(self) -> Self::ByteArray{
					match self{
						#( #match_arms_rev )*
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Creates an enum with unit variants from the derived enum, and implements `enum_traits::Tag`.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// #[derive(EnumTag)]
/// enum Enum{
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
/// assert_eq!(EnumTag::Dog  ,Enum::Dog.tag());
/// assert_eq!(EnumTag::Cat  ,Enum::Cat(0).tag());
/// assert_eq!(EnumTag::Robot,Enum::Robot{speed: 0.0}.tag());
/// # }
/// ```
#[proc_macro_derive(EnumTag)]
pub fn derive_EnumTag(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ref visibility = item.vis;

		let unit_enum_ident = Ident::from({
			const SUFFIX: &'static str = "Tag";
			let mut str = String::with_capacity(ident.as_ref().len() + SUFFIX.len());
			str.push_str(ident.as_ref());
			str.push_str(SUFFIX);
			str
		});

		let match_arms = data.iter().map(|variant|{
			let variant_ident = &variant.ident;

			match variant.data {
				VariantData::Unit => {
					quote! { &#ident::#variant_ident     => #unit_enum_ident::#variant_ident, }
				}
				VariantData::Tuple(_) => {
					quote! { &#ident::#variant_ident(..) => #unit_enum_ident::#variant_ident, }
				}
				VariantData::Struct(_) => {
					quote! { &#ident::#variant_ident{..} => #unit_enum_ident::#variant_ident, }
				}
			}
		});

		let unit_variants = data.iter().map(|variant|{
			let variant_ident = &variant.ident;
			quote! { #variant_ident, }
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
			#visibility enum #unit_enum_ident{
				#( #unit_variants )*
			}

			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::enum_traits::Tag for #ident #ty_generics #where_clause{
				type Enum = #unit_enum_ident;

				#[inline]
				fn tag(&self) -> Self::Enum{
					match self{
						#( #match_arms )*
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

/// Implements functions that checks if the current state of the enum is a certain variant.
///
/// # Requirements
/// - The derived item is an enum
///
/// # Examples
///
/// ```rust
/// # #![feature(associated_consts)]
/// # #[macro_use]extern crate enum_traits_macros;
/// # extern crate enum_traits;
/// # use enum_traits::*;
/// # fn main(){
/// #[derive(EnumIsVariantFns)]
/// enum Enum {
/// 	Dog,
/// 	Cat(i32),
/// 	Robot{speed: f32},
/// }
/// assert!(Enum::Dog.is_dog());
/// assert!(Enum::Cat(0).is_cat());
/// assert!(Enum::Robot{speed: 0.0}.is_robot());
///
/// assert!(!Enum::Dog.is_cat());
/// assert!(!Enum::Dog.is_robot());
/// assert!(!Enum::Robot{speed: 0.0}.is_cat());
/// assert!(!Enum::Robot{speed: 0.0}.is_dog());
/// assert!(!Enum::Cat(0).is_dog());
/// assert!(!Enum::Cat(0).is_robot());
/// # }
/// ```
#[proc_macro_derive(EnumIsVariantFns)]
pub fn derive_EnumIsVariantFns(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();


		let fns = data.iter().map(|variant|{
			let fn_ident = Ident::from({
				const PREFIX: &'static str = "is_";
				let mut str = String::with_capacity(variant.ident.as_ref().len() + PREFIX.len());
				str.push_str(PREFIX);
				str.push_str(variant.ident.as_ref().to_ascii_lowercase().as_ref());
				str
			});

			let pattern = {
				let variant_ident = &variant.ident;
				match variant.data{
					VariantData::Unit => {
						quote! { #ident::#variant_ident }
					}
					VariantData::Tuple(_) => {
						quote! { #ident::#variant_ident(..) }
					}
					VariantData::Struct(_) => {
						quote! { #ident::#variant_ident{..} }
					}
				}
			};

			quote! {
				#[inline(always)]
				#[allow(dead_code)]
				pub fn #fn_ident(&self) -> bool{
					if let &#pattern = self{true}else{false}
				}
			}
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #fns )*
			}
		}
	}
	derive_enum(input,gen_impl)
}
