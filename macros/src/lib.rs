#![allow(non_snake_case)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use std::{cmp,iter};
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

#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: TokenStream) -> TokenStream{
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

#[proc_macro_derive(EnumIter)]
pub fn derive_EnumIter(input: TokenStream) -> TokenStream{
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

		let impl_diter = quote!{
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
		};

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
			#impl_diter
			#impl_eiter
		}
	}
	derive_enum(input,gen_impl)
}

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
