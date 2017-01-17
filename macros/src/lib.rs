#![allow(non_snake_case)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{Attribute,Body,Ident,Lit,IntTy,MacroInput,Variant,VariantData};
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
		MetaItem::List(ref ident,ref content) if ident=="attr" && content.len()==1 => match &content[0]{
			&NestedMetaItem::MetaItem(MetaItem::Word(ref ty)) if ty!="C" => return Some(ty.clone()),
			_ => continue,
		},
		_ => continue,
	}}
	None
}

fn variant_unit_ident(variant: &Variant) -> &Ident{match variant.data{
	VariantData::Unit => {
		&variant.ident
	}
	_ => panic!("`derive(Enum_?)` may only be applied to enum items with no fields")
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

#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: TokenStream) -> TokenStream{
	fn gen_impl(ident: &Ident,item: &MacroInput,data: &Vec<Variant>) -> Tokens{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let len = data.len();

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
		let ty = type_from_repr_attr(item.attrs.iter()).unwrap_or_else(|| minimum_type_from_value(data.len()));

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

		fn match_arm_transform(ident: &Ident,variant: &Variant) -> Option<Tokens>{
			let variant_ident = &variant.ident;

			variant.discriminant.as_ref().map(|ref variant_discriminant|{
				match variant.data{
					VariantData::Unit => {
						quote! { #variant_discriminant => #ident::#variant_ident, }
					}
					VariantData::Tuple(_) => {
						quote! { #variant_discriminant::#variant_ident(..) => #ident::#variant_ident, }
					}
					VariantData::Struct(_) => {
						quote! { #variant_discriminant::#variant_ident{..} => #ident::#variant_ident, }
					}
				}
			})
		};
		let match_arms1 = data.iter().filter_map(|variant| match_arm_transform(ident,variant));
		let match_arms2 = data.iter().filter_map(|variant| match_arm_transform(ident,variant));
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
