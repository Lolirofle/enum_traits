#![feature(proc_macro,proc_macro_lib)]
#![allow(non_snake_case)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{Body,Generics,Ident,Variant,VariantData};
use quote::Tokens;

#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();
		let len = data.len();

		quote!{
			impl #impl_generics ::enum_traits::Len for #ident #ty_generics #where_clause{
				const LEN: usize = #len;
			}
		}
	}
	derive_enum(input,gen_impl)
}

#[proc_macro_derive(EnumEnds)]
pub fn derive_EnumEnds(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();
		let variant_first_ident = &data.first().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;
		let variant_last_ident  = &data.last().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;

		quote!{
			impl #impl_generics ::enum_traits::Ends for #ident #ty_generics #where_clause{
				#[inline(always)]fn first() -> Self{#ident::#variant_first_ident}
				#[inline(always)]fn last()  -> Self{#ident::#variant_last_ident}
			}
		}
	}
	derive_enum(input,gen_impl)
}

#[proc_macro_derive(EnumToIndex)]
pub fn derive_EnumToIndex(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();

		let match_arms = data.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;

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

#[proc_macro_derive(EnumFromIndex)]
pub fn derive_EnumFromIndex(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();

		let match_arms = data.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;

			match variant.data{
				VariantData::Unit => {
					quote! { #i => Some(#ident::#variant_ident), }
				}
				_ => panic!("`derive(EnumFromIndex)` may only be applied to enum items with no fields")
			}
		});

		quote!{
			impl #impl_generics ::enum_traits::FromIndex for #ident #ty_generics #where_clause{
				fn from_index(index: <Self as Index>::Type) -> Option<Self>{
					match index{
						#( #match_arms )*
						_ => None
					}
				}
			}
		}
	}
	derive_enum(input,gen_impl)
}

#[proc_macro_derive(EnumIndex)]
pub fn derive_EnumIndex(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();

		quote!{
			impl #impl_generics ::enum_traits::Index for #ident #ty_generics #where_clause{
				type Type = usize;
			}
		}
	}
	derive_enum(input,gen_impl)
}

#[proc_macro_derive(EnumIter)]
pub fn derive_EnumIter(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,generics: &Generics,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = generics.split_for_impl();

		let len = data.len();

		fn map_variant_ident(variant: &Variant) -> &Ident{match variant.data{
			VariantData::Unit => {
				&variant.ident
			}
			_ => panic!("`derive(EnumIter)` may only be applied to enum items with no fields")
		}}

		let prev_match_arms = {
				let iter = data.iter().rev().map(map_variant_ident);
				iter.zip(data.iter().rev().map(map_variant_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});

		let next_match_arms = {
				let iter = data.iter().map(map_variant_ident);
				iter.zip(data.iter().map(map_variant_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});

		quote!{
			impl #impl_generics ::std::iter::Iterator for #ident #ty_generics #where_clause{
				type Item = Self;
				#[inline]
				fn next(&mut self) -> Option<Self>{
					Some(match self{
						#( #next_match_arms )*
						_ => return None
					})
				}
				#[inline(always)]
				fn size_hint(&self) -> (usize,Option<usize>){
					use ::std::iter::ExactSizeIterator;
					(self.len(),Some(self.len()))
				}
			}

			impl #impl_generics ::std::iter::DoubleEndedIterator for #ident #ty_generics #where_clause{
				#[inline]fn next_back(&mut self) -> Option<Self::Item>{
					Some(match self{
						#( #prev_match_arms )*
						_ => return None
					})
				}
			}

			impl #impl_generics ::std::iter::ExactSizeIterator for #ident #ty_generics #where_clause{
				#[inline(always)]fn len(&self) -> usize{#len}
			}
		}
	}
	derive_enum(input,gen_impl)
}

fn derive_enum(input: TokenStream,gen_impl: fn(&Ident,&Generics,&Vec<Variant>) -> Tokens) -> TokenStream{
	let input = input.to_string();
	let ast = syn::parse_macro_input(&input).unwrap();

	let quote_tokens = match ast.body{
		Body::Enum(ref data) => gen_impl(&ast.ident,&ast.generics,data),
		_ => panic!("`derive(Enum_?)` may only be applied to enum items")
	}.to_string();

	format!("{}{}",input,quote_tokens.to_string()).parse().unwrap()
}
