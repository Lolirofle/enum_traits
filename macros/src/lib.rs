#![allow(non_snake_case)]

#![cfg_attr(feature = "no_std_compile" ,no_std)]
#![cfg_attr(feature = "no_std_compile" ,feature(alloc))]

#[cfg(feature = "no_std_compile")]extern crate alloc;
extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;
use proc_macro2::{Literal,Span,TokenStream};

#[cfg(not(feature = "no_std_compile"))]use  std::{cmp,iter};
#[cfg(not(feature = "no_std_compile"))]use  std::iter::FromIterator;
#[cfg(feature = "no_std_compile")     ]use core::{cmp,iter};
#[cfg(feature = "no_std_compile")     ]use core::iter::FromIterator;
#[cfg(feature = "no_std_compile")     ]use alloc::string::{String,ToString};
#[cfg(feature = "no_std_compile")     ]use alloc::vec::Vec;
use syn::{Attribute,Expr,ExprLit,Fields,Ident,ItemEnum,Lit,PathSegment,Variant};

#[cfg(not(feature = "no_std"))]const STD: &'static str = "std";
#[cfg(feature = "no_std")     ]const STD: &'static str = "core";

fn minimum_type_from_value(value: usize) -> Ident{
	if value <= u8::max_value() as usize{
		Ident::new("u8",Span::call_site())
	}else if value <= u16::max_value() as usize{
		Ident::new("u16",Span::call_site())
	}else if value <= u32::max_value() as usize{
		Ident::new("u32",Span::call_site())
	}else if value <= u64::max_value() as usize{
		Ident::new("u64",Span::call_site())
	}else{
		Ident::new("usize",Span::call_site())
	}
}

/**
 * Extracts the type from `repr(u*)` or `repr(i*)` attributes if it exists.
 */
fn type_from_repr_attr<'i,I>(attrs: I) -> Option<Ident>
	where I: Iterator<Item = &'i Attribute>
{
	use syn::{Meta,MetaList,NestedMeta};

	for attr in attrs{match attr.parse_meta(){
		Ok(Meta::List(MetaList{path,nested,..})) if path.is_ident("repr") => {
			for meta in nested{
				if let NestedMeta::Meta(Meta::Path(repr)) = meta{
					if let Some(repr) = repr.get_ident(){
						let repr = repr.to_string();
						if repr.as_str() == "usize" || repr.as_str() == "isize" || {
							let mut repr_chars = repr.chars();
							repr_chars.next().map_or(false , |c| c == 'u' || c == 'i') //Starts with an 'u' or 'i'.
							&& repr_chars.next().map_or(false , |c| c.is_digit(10)) //Exists a digit after.
							&& repr_chars.all(|c| c.is_digit(10)) //All after are digits too.
						}{
							return Some(Ident::new(repr.as_ref(),Span::call_site()));
						}
					}
				}
				continue;
			}
		},
		_ => {continue;},
	}}
	None
}

fn variant_unit_ident<'v>(variant: &'v Variant,derive_name: &'static str) -> &'v Ident{
	match variant.fields{
		Fields::Unit => {
			&variant.ident
		}
		_ => panic!("`derive({})` may only be applied to enum items with no fields",derive_name)
	}
}

#[inline(always)]
fn derive_enum<F>(input: proc_macro::TokenStream,gen_impl: F) -> proc_macro::TokenStream
	where F: FnOnce(ItemEnum,PathSegment) -> TokenStream
{
	let input = proc_macro2::TokenStream::from(input);
	let item = syn::parse2::<ItemEnum>(input).expect("`derive(Enum*)` may only be applied to enum items");
	proc_macro::TokenStream::from(gen_impl(item,PathSegment::from(Ident::new(STD,Span::call_site()))))
}

fn minimum_type_containing_enum(item: &ItemEnum) -> syn::Ident{//TODO: Maybe useful to export?
	//First, check if there's a repr attribute
	type_from_repr_attr(item.attrs.iter())
	.unwrap_or_else(||
		//Second, use the maximum value of an explicit discriminant or the length of the enum (depending on which is the greatest)
		minimum_type_from_value(match item.variants.iter().filter_map(|variant| match variant.discriminant{
				Some((_,Expr::Lit(ExprLit{lit: Lit::Int(ref discrimimant) , ..}))) => Some(discrimimant.base10_parse::<usize>().expect("Discriminant cannot be made into an usize")),
				_ => None
			}).max(){
				Some(max) => cmp::max(cmp::max(item.variants.len(),1)-1 , max),
				//Third, use the length of the enum
				_ => cmp::max(item.variants.len(),1)-1
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
/// 	assert_eq!(0,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A}
/// 	assert_eq!(1,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C}
/// 	assert_eq!(3,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G}
/// 	assert_eq!(7,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H}
/// 	assert_eq!(8,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,X,Y,Z}
/// 	assert_eq!(25,T::len());
/// }{
/// 	#[derive(EnumLen)]enum T{A,B(),C{},D(u8),E{e: u8},F(u8,u16),G{g1: u8,g2: u16},H}
/// 	assert_eq!(8,T::len());
/// }
/// # }
/// ```
#[proc_macro_derive(EnumLen)]
pub fn derive_EnumLen(input: proc_macro::TokenStream) -> proc_macro::TokenStream{ //TODO: Consider allowing structs. Number of variants of struct is always 1
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;
		let len = item.variants.len();

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
pub fn derive_EnumEnds(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;
		let variant_first_ident = &item.variants.first().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;
		let variant_last_ident  = &item.variants.last().expect("`derive(EnumEnds)` may only be applied to non-empty enums").ident;

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
pub fn derive_EnumToIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());

			match variant.fields{
				Fields::Unit => {
					quote! { &#ident::#variant_ident => #i, }
				}
				Fields::Unnamed(_) => {
					quote! { &#ident::#variant_ident(..) => #i, }
				}
				Fields::Named(_) => {
					quote! { &#ident::#variant_ident{..} => #i, }
				}
			}
		});

		let match_arms_into = item.variants.iter().enumerate().map(|(i,variant)|{
			let variant_ident = &variant.ident;
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());

			match variant.fields{
				Fields::Unit => {
					quote! { #ident::#variant_ident => #i, }
				}
				Fields::Unnamed(_) => {
					quote! { #ident::#variant_ident(..) => #i, }
				}
				Fields::Named(_) => {
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
pub fn derive_EnumFromIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumFromIndex")
	}

	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = &item.ident;

		fn match_arm_transform(ident: &Ident,(i,variant_ident): (usize,&Ident)) -> TokenStream{
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());
			quote! { #i => #ident::#variant_ident, }
		}
		let match_arms1 = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));
		let match_arms2 = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));

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
						_ => unreachable!()
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
pub fn derive_EnumIndex(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = &item.ident;

		//Determine which type to use (attribute or number of variants)
		let ty = type_from_repr_attr(item.attrs.iter())
			.unwrap_or_else(|| minimum_type_from_value(cmp::max(item.variants.len(),1)-1));

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
pub fn derive_EnumIter(input: proc_macro::TokenStream) -> proc_macro::TokenStream{//TODO: Consider rewriting output (EnumIter may not need Option, but then empty enums are not represented. Are they necessary to include?)
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumIter")
	}

	fn gen_impl(item: ItemEnum,std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;
		let visibility = &item.vis;

		let len = item.variants.len();
		let last = item.variants.last();

		/*let prev_match_arms = {
				let iter = item.variants.iter().rev().map(variant_unit_ident);
				iter.zip(item.variants.iter().rev().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
			});*/

		let next_match_arms = {
				let iter = item.variants.iter().map(variant_unit_ident);
				iter.zip(item.variants.iter().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
			});

		let len_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { &#ident::#variant_ident => #i, }
		});

		let count_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { #ident::#variant_ident => #i, }
		});

		let variant_first_ident = &item.variants.first().expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;
		//let variant_last_ident  = &item.variants.last().expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;

		let struct_ident = {
			let mut str = ident.to_string().to_string();
			str.push_str("Iter");
			Ident::new(str.as_ref(),Span::call_site())
		};

		let struct_iter = quote!{
			#visibility struct #struct_ident #ty_generics #where_clause (pub Option<#ident #ty_generics>);
		};

		let impl_default = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::#std::default::Default for #struct_ident #ty_generics #where_clause{
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
					use ::#std::iter::ExactSizeIterator;
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
				impl #impl_generics ::#std::iter::Iterator for #struct_ident #ty_generics #where_clause{
					type Item = #ident;

					#fn_next
					#fn_size_hint
					#fn_count
					#fn_last
				}
			}
		};

		//TODO: May be an incorrect use of DoubleEndedIterator. Use Step instead
		/*let impl_diter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::#std::iter::Iterator for ::#std::iter::Rev<#struct_ident> #ty_generics #where_clause{
				#[inline]
				fn next_back(&mut self) -> Option<Self::Item>{
					Some(match &self.0{
						&None => {self.0 = Some(#ident::#variant_last_ident); #ident::#variant_last_ident},
						#( #prev_match_arms )*
						_ => return None
					})
				}
			}
		};*/

		let impl_exactiter = quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::#std::iter::ExactSizeIterator for #struct_ident #ty_generics #where_clause{
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
			//#impl_diter
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
pub fn derive_EnumIterator(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumIterator")
	}

	fn gen_impl(item: ItemEnum,std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let len = item.variants.len();
		let last = item.variants.last();

		/*let prev_match_arms = {
				let iter = item.variants.iter().rev().map(variant_unit_ident);
				iter.zip(item.variants.iter().rev().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});*/

		let next_match_arms = {
				let iter = item.variants.iter().map(variant_unit_ident);
				iter.zip(item.variants.iter().map(variant_unit_ident).skip(1))
			}.map(|(variant_ident1,variant_ident2)|{
				quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
			});

		let len_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());
			let variant_ident: &Ident = variant_unit_ident(variant);
			quote! { &#ident::#variant_ident => #i, }
		});

		let count_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
			let i = Lit::Int(Literal::usize_unsuffixed(i).into());
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
					use ::#std::iter::ExactSizeIterator;
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
				impl #impl_generics ::#std::iter::Iterator for #ident #ty_generics #where_clause{
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
			impl #impl_generics ::#std::iter::DoubleEndedIterator for #ident #ty_generics #where_clause{
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
			impl #impl_generics ::#std::iter::ExactSizeIterator for #ident #ty_generics #where_clause{
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
/// - The enum variants are all unit variants
#[proc_macro_derive(EnumDiscriminant)]
pub fn derive_EnumDiscriminant(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = &item.ident;

		fn variant_to_match_arm(ident: &Ident,variant: &Variant,only_unit_variants: bool) -> Option<TokenStream>{
			let variant_ident = &variant.ident;

			//If an explicit discriminant exists
			variant.discriminant.as_ref().map(|(_,ref variant_discriminant)|{
				match variant.fields{
					Fields::Unit => {
						quote! { #variant_discriminant => #ident::#variant_ident, }
					}
					Fields::Unnamed(_)  |
					Fields::Named(_) => {
						//Tuple and struct variants cannot have explicit discriminants
						unreachable!()
					}
				}
			}).or_else(||{
				match variant.fields{
					Fields::Unit if only_unit_variants => Some({
						quote! { n if n==#ident::#variant_ident as Self::Type => #ident::#variant_ident, }
					}),
					_ => None
				}
			})
		}
		let only_unit_variants = item.variants.iter().all(|variant| match variant.fields{Fields::Unit => true , _ => false});
		let match_arms1 = item.variants.iter().filter_map(|variant| variant_to_match_arm(ident,variant,only_unit_variants));
		let match_arms2 = item.variants.iter().filter_map(|variant| variant_to_match_arm(ident,variant,only_unit_variants));
		let ty = type_from_repr_attr(item.attrs.iter()).unwrap_or(Ident::new("usize",Span::call_site()));

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
						_ => ::#std::mem::uninitialized()
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
pub fn derive_EnumVariantName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream {
		let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let match_arms = item.variants.iter().map(|variant| {
			let variant_ident = &variant.ident;
			let variant_str = variant.ident.to_string();

			match variant.fields {
				Fields::Unit => {
					quote! { &#ident::#variant_ident => #variant_str, }
				}
				Fields::Unnamed(_) => {
					quote! { &#ident::#variant_ident(..) => #variant_str, }
				}
				Fields::Named(_) => {
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
pub fn derive_EnumFromVariantName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {//TODO: Consider not using FromStr, instead an own trait
	fn gen_impl(item: ItemEnum,std: PathSegment) -> TokenStream {
		let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let match_arms = item.variants.iter().filter_map(|variant| {
			let variant_ident = &variant.ident;
			let variant_str = variant.ident.to_string();

			if let Fields::Unit = variant.fields{
				Some(quote! { #variant_str => #ident::#variant_ident, })
			}else{
				None
			}
		});

		quote!{
			#[automatically_derived]
			#[allow(unused_attributes)]
			impl #impl_generics ::#std::str::FromStr for #ident #ty_generics #where_clause{
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

/*
/// Implements `enum_traits::BitPattern`.
///
/// # Requirements
/// - The derived item is an enum
#[proc_macro_derive(EnumBitPattern)]
pub fn derive_EnumBitPattern(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn variant_unit_ident(variant: &Variant) -> &Ident{
		::variant_unit_ident(variant,"EnumBitPattern")
	}

	fn gen_impl(item: ItemEnum,std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let n = (item.variants.len() as f64 / 8.0).ceil() as usize;
		fn match_arm_transform(ident: &Ident,(i,variant_ident): (usize,&Ident),n: usize) -> TokenStream{
			let lit = Punctuated::from_iter({ //TODO: Do not convert from iter to vec and then to iter again
				let mut l = Vec::from_iter(iter::repeat(Expr::from(Lit::Int(LitInt::new(0 as u64,Span::call_site())))).take(n));
				l[n-i/8-1] = Expr::from(Lit::Int(LitInt::new((0b00000001u8.rotate_left((i as u32)%8) as u64),Span::call_site())));
				l.into_iter()
			});
			quote! { #ident::#variant_ident => [ #lit ], }
		}
		fn match_arm_transform_rev(ident: &Ident,(i,variant_ident): (usize,&Ident),n: usize) -> TokenStream{
			let lit = Punctuated::from_iter({
				let mut l = Vec::from_iter(iter::repeat(Expr::from(Lit::Int(LitInt::new(0 as u64,Span::call_site())))).take(n));
				l[i/8] = Expr::from(Lit::Int(LitInt::new((0b10000000u8.rotate_right((i as u32)%8) as u64),Span::call_site())));
				l.into_iter()
			});
			quote! { #ident::#variant_ident => [ #lit ], }
		}
		let match_arms     = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg,n));
		let match_arms_rev = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform_rev(ident,arg,n));

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
*/

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
pub fn derive_EnumTag(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;
		let ref visibility = item.vis;

		let unit_enum_ident = Ident::new(format!("{}Tag",ident.to_string()).as_ref(),Span::call_site());

		let match_arms = item.variants.iter().map(|variant|{
			let variant_ident = &variant.ident;

			match variant.fields {
				Fields::Unit => {
					quote! { &#ident::#variant_ident     => #unit_enum_ident::#variant_ident, }
				}
				Fields::Unnamed(_) => {
					quote! { &#ident::#variant_ident(..) => #unit_enum_ident::#variant_ident, }
				}
				Fields::Named(_) => {
					quote! { &#ident::#variant_ident{..} => #unit_enum_ident::#variant_ident, }
				}
			}
		});

		let unit_variants = item.variants.iter().map(|variant|{
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
#[cfg(not(feature = "no_std_compile"))]
#[proc_macro_derive(EnumIsVariantFns)]
pub fn derive_EnumIsVariantFns(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
	fn gen_impl(item: ItemEnum,_std: PathSegment) -> TokenStream{
		let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
		let ident = item.ident;

		let fns = item.variants.iter().map(|variant|{
			let fn_ident = Ident::new(format!("is_{}",variant.ident.to_string().to_ascii_lowercase()).as_ref(),Span::call_site());

			let pattern = {
				let variant_ident = &variant.ident;
				match variant.fields{
					Fields::Unit => {
						quote! { #ident::#variant_ident }
					}
					Fields::Unnamed(_) => {
						quote! { #ident::#variant_ident(..) }
					}
					Fields::Named(_) => {
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
