use alloc::vec::Vec;
use core::iter;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub enum FieldPos<'i>{
	Unnamed(usize),
	Named(&'i syn::Ident),
}

impl<'i> FieldPos<'i>{
	fn to_pat(&self,variant_path: &syn::Path,ident: &syn::Ident) -> syn::Pat{match self{
		FieldPos::Unnamed(i) => {
			let prior = iter::repeat_n::<syn::Pat>(syn::parse_quote!{ _ },*i);
			syn::parse_quote!{ #variant_path(#( #prior , )* #ident,..) }
		},
		FieldPos::Named(i) => syn::parse_quote!{ #variant_path{#i: #ident,..} },
	}}

	fn gen_match_arm<'v>(self,enum_ident: &syn::Ident,variant: &'v syn::Variant) -> TokenStream{
		let variant_ident = &variant.ident;
		let var_ident = &syn::Ident::new("x",proc_macro2::Span::mixed_site());
		let pat = self.to_pat(&syn::parse_quote!{ #enum_ident::#variant_ident },var_ident);
		quote!{ #pat => #var_ident, }
	}
}

fn find_field_by_type<'f>(fields: &'f syn::Fields,ty: &syn::Type,preferred_names: impl Iterator<Item = &'f syn::Ident>) -> Option<FieldPos<'f>>{
	match fields{
		syn::Fields::Unnamed(f) => f.unnamed.iter()
			.position(|f| &f.ty == ty)
			.map(FieldPos::Unnamed),
		syn::Fields::Named(f) => {
			//Prefer both the same name and type before only checking type
			for name in preferred_names{
				if let Some(f) = f.named.iter().find(|f| f.ident.as_ref() == Some(name) && &f.ty == ty){
					return Some(FieldPos::Named(f.ident.as_ref().unwrap()));
				}
			}
			f.named.iter()
				.find(|f| &f.ty == ty)
				.map(|f| FieldPos::Named(f.ident.as_ref().unwrap()))
		}
		syn::Fields::Unit => None,
	}
}

#[cfg(feature = "attr_into")]
fn find_field_by_name<'f>(fields: &'f syn::Fields,ty: Option<&syn::Type>,name: &syn::Ident) -> Option<(&'f syn::Type,FieldPos<'f>)>{
	match fields{
		syn::Fields::Named(f) if let Some(f) = f.named.iter().find(|f| f.ident.as_ref() == Some(name) && ty.map_or(true,|ty| &f.ty == ty)) =>
			Some((&f.ty,FieldPos::Named(f.ident.as_ref().unwrap()))),
		_ => None,
	}
}

#[cfg(feature = "attr_into")]
fn find_field_by_index<'f>(fields: &'f syn::Fields,i: usize) -> Option<(&'f syn::Field,FieldPos<'f>)>{
	match fields{
		syn::Fields::Unnamed(f) => f.unnamed.get(i).map(|f| (f,FieldPos::Unnamed(i))),
		syn::Fields::Named(f) => f.named.get(i).map(|f| (f,FieldPos::Named(f.ident.as_ref().unwrap()))),
		syn::Fields::Unit => None,
	}
}

#[cfg(feature = "derive_into")]
fn count_named_fields<'v>(variants: impl Iterator<Item = &'v syn::Variant>) -> Vec<(&'v syn::Ident,usize)>{
    let mut counts: Vec<(&'v syn::Ident,usize)> = Vec::new();
    for variant in variants{
        let syn::Fields::Named(fields) = &variant.fields else {continue};
        for field in &fields.named{
            let Some(ref name) = field.ident else{continue};
            match counts.iter_mut().find(|(n,_)| *n == name){
                Some((_, c)) => *c += 1,
                None => counts.push((name,1)),
            }
        }
    }

    counts.sort_by(|a,b| b.1.cmp(&a.1));
    counts
}
#[cfg(feature = "derive_into")]
fn common_field_types<'v>(variants: impl Iterator<Item = &'v syn::Variant> + ExactSizeIterator + Clone) -> Vec<(syn::Type,Vec<(&'v syn::Variant,FieldPos<'v>)>)>{
	let max_size = variants.len();
	let mut out = Vec::new();
	let names = count_named_fields(variants.clone());
	if let Some(first) = variants.clone().next(){
		//Collect the unique initial types from the first variant.
		for field in match first.fields{
			syn::Fields::Named(ref f)   => &f.named,
			syn::Fields::Unnamed(ref f) => &f.unnamed,
			syn::Fields::Unit           => return out,
		}.iter(){
			if out.iter().all(|(ty,_)| ty != &field.ty){
				out.push((field.ty.clone(),Vec::with_capacity(max_size)))
			}
		}

		//Intersection.
		for variant in variants{
			out.retain_mut(|(ty,poss)|
				match find_field_by_type(&variant.fields,ty,names.iter().cloned().map(|(x,_)| x)){
					Some(pos) => {poss.push((variant,pos)); true},
					None => false
				}
			);
		}
	}
	out
}

#[cfg(feature = "derive_into")]
fn unique_field_types<'v>(variants: impl Iterator<Item = &'v syn::Variant> + ExactSizeIterator) -> Vec<(syn::Type,Vec<(&'v syn::Variant,FieldPos<'v>)>)>{
	let max_size = variants.len();
	let mut out = Vec::<(syn::Type,Vec<(&'v syn::Variant,FieldPos<'v>)>)>::new();
	for variant in variants{
		for (f,field) in match variant.fields{
			syn::Fields::Named(ref f)   => &f.named,
			syn::Fields::Unnamed(ref f) => &f.unnamed,
			syn::Fields::Unit           => continue,
		}.iter().enumerate(){
			let pos = (variant,match field.ident{
				Some(ref i) => FieldPos::Named(i),
				None => FieldPos::Unnamed(f),
			});

			match out.iter_mut().find(|(ty,_)| ty == &field.ty){
				Some((_,o)) => o.push(pos),
				None => out.push((field.ty.clone(),{let mut o = Vec::with_capacity(max_size); o.push(pos); o})),
			}
		}
	}
	out
}

#[cfg(feature = "derive_into")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let impls = common_field_types(item.variants.iter());
	if impls.len() == 0{return syn::Error::new(item.span(),"`derive(EnumInto)` is applied to an enum without any common types shared by all its fields.").into_compile_error();}
	let impls = impls.into_iter().map(|(ty,poss)|{
		let match_arms = poss.into_iter().map(|(variant,pos)| pos.gen_match_arm(ident,variant));
		quote!{
			#[automatically_derived]
			impl #impl_generics ::core::convert::From<#ident #ty_generics> for #ty #where_clause{
				fn from(_x: #ident #ty_generics) -> Self{
					match _x{
						#( #match_arms )*
					}
				}
			}
		}
	});

	quote!{#( #impls )*}
}

#[cfg(feature = "derive_into")]
pub fn gen_derive_try(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let impls = unique_field_types(item.variants.iter()).into_iter().map(|(ty,poss)|{
		let match_arms = poss.into_iter().map(|(variant,pos)| pos.gen_match_arm(ident,variant));
		quote!{
			#[automatically_derived]
			impl #impl_generics ::core::convert::TryFrom<#ident #ty_generics> for #ty #where_clause{
				type Error = ();
				fn try_from(_x: #ident #ty_generics) -> ::core::result::Result<Self,Self::Error>{
					::core::result::Result::Ok(match _x{
						#( #match_arms )*
						_ => return ::core::result::Result::Err(())
					})
				}
			}
		}
	});

	quote!{#( #impls )*}
}

#[cfg(feature = "attr_into")] use crate::util::try_tokenstream;
#[cfg(feature = "attr_into")] use crate::util::parse::{Concat,Delimited,ItemPrefix};

#[cfg(feature = "attr_into")]
pub fn gen_attr_names(
	Delimited(params): Delimited<Vec<syn::MetaList>>,
	item: syn::ItemEnum
) -> TokenStream{
	use crate::util;
	use crate::util::replace_ty::replace_infer_ret;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let fns = params.iter().map(|param|{
		if param.path.is_ident("name"){
			let Concat((ref field_ident,_,ItemPrefix(attrs,vis,mut sign))): Concat<(syn::Ident,syn::Token![=>],ItemPrefix<syn::Signature>)> = try_tokenstream!(param.parse_args());

			let never_ty = syn::parse_quote!{!};
			let ty = item.variants
				.iter()
				.find_map(|variant| find_field_by_name(&variant.fields,None,field_ident).map(|(ty,_)| ty))
				.unwrap_or(&never_ty)
			;
			sign.output = replace_infer_ret(sign.output,ty);
			let match_arms = item.variants.iter().filter_map(|variant|{
				find_field_by_name(&variant.fields,None,field_ident).map(|(_,pos)| pos.gen_match_arm(ident,variant))
			});

			if util::is_option_ret(&sign.output){quote!{
				#( #attrs )* #vis #sign{
					::core::option::Option::Some(match self{#( #match_arms )* _ => return ::core::option::Option::None})
				}
			}}else{quote!{
				#( #attrs )* #vis #sign{
					match self{#( #match_arms )*}
				}
			}}
		}else if param.path.is_ident("ty"){
			let Concat((ref ty,_,ItemPrefix(attrs,vis,mut sign))): Concat<(syn::Type,syn::Token![=>],ItemPrefix<syn::Signature>)> = try_tokenstream!(param.parse_args());
			sign.output = replace_infer_ret(sign.output,ty);

			let match_arms = item.variants.iter().filter_map(|variant|{
				find_field_by_type(&variant.fields,ty,iter::empty()).map(|pos| pos.gen_match_arm(ident,variant))
			});

			if util::is_option_ret(&sign.output){quote!{
				#( #attrs )* #vis #sign{
					::core::option::Option::Some(match self{#( #match_arms )* _ => return ::core::option::Option::None})
				}
			}}else{quote!{
				#( #attrs )* #vis #sign{
					match self{#( #match_arms )*}
				}
			}}
		}else if param.path.is_ident("index"){
			let Concat((ref field_index,_,ItemPrefix(attrs,vis,mut sign))): Concat<(syn::Index,syn::Token![=>],ItemPrefix<syn::Signature>)> = try_tokenstream!(param.parse_args());

			let never_ty = syn::parse_quote!{!};
			let ty = item.variants
				.iter()
				.find_map(|variant| find_field_by_index(&variant.fields,field_index.index as usize).map(|(field,_)| &field.ty))
				.unwrap_or(&never_ty)
			;
			sign.output = replace_infer_ret(sign.output,ty);
			let match_arms = item.variants.iter().filter_map(|variant|{
				find_field_by_index(&variant.fields,field_index.index as usize).map(|(_,pos)| pos.gen_match_arm(ident,variant))
			});

			if util::is_option_ret(&sign.output){quote!{
				#( #attrs )* #vis #sign{
					::core::option::Option::Some(match self{#( #match_arms )* _ => return ::core::option::Option::None})
				}
			}}else{quote!{
				#( #attrs )* #vis #sign{
					match self{#( #match_arms )*}
				}
			}}
		}else{
			return syn::Error::new(param.path.span(),"`impl_enum_into` is supplied an unknown parameter. Expected `name`, `ty`, `index`.").into_compile_error();
		}
	});
	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fns )*
		}
	}
}
