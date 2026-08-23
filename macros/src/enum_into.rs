use alloc::vec;
use alloc::vec::Vec;
use core::iter;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub enum FieldPos{
	Unnamed(usize),
	Named(syn::Ident),
}

impl FieldPos{
	fn to_pat(&self,variant_path: &syn::Path,ident: &syn::Ident) -> syn::Pat{match self{
		FieldPos::Unnamed(i) => {
			let prior = iter::repeat_n::<syn::Pat>(syn::parse_quote!{ _ },*i);
			syn::parse_quote!{ #variant_path(#( #prior , )* #ident,..) }
		},
		FieldPos::Named(i) => syn::parse_quote!{ #variant_path{#i: #ident,..} },
	}}
}

#[inline]
fn find_field_by_type<'n>(fields: &syn::Fields,ty: &syn::Type,preferred_names: impl Iterator<Item = &'n syn::Ident>) -> Option<FieldPos>{
	match fields{
		syn::Fields::Unnamed(f) => f.unnamed.iter()
			.position(|f| &f.ty == ty)
			.map(FieldPos::Unnamed),
		syn::Fields::Named(f) => {
			//Prefer both the same name and type before only checking type
			for name in preferred_names{
				if let Some(f) = f.named.iter().find(|f| f.ident.as_ref() == Some(name) && &f.ty == ty){
					return Some(FieldPos::Named(f.ident.clone().unwrap()));
				}
			}
			f.named.iter()
				.find(|f| &f.ty == ty)
				.map(|f| FieldPos::Named(f.ident.clone().unwrap()))
		}
		syn::Fields::Unit => None,
	}
}

#[inline]
fn find_field_by_name(fields: &syn::Fields,ty: &syn::Type,name: &syn::Ident) -> Option<FieldPos>{
	match fields{
		syn::Fields::Named(f) if let Some(f) = f.named.iter().find(|f| f.ident.as_ref() == Some(name) && &f.ty == ty) =>
			Some(FieldPos::Named(f.ident.clone().unwrap())),
		_ => None,
	}
}

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

fn common_field_types<'v>(mut variants: impl Iterator<Item = &'v syn::Variant> + Clone) -> Vec<(syn::Type,Vec<FieldPos>)>{
	let mut out = Vec::new();
	let names = count_named_fields(variants.clone());
	if let Some(first) = variants.next(){
		match first.fields{
			syn::Fields::Named(ref f)   => out.extend(f.named.iter().map(|f| (f.ty.clone(),vec![FieldPos::Named(f.ident.as_ref().unwrap().clone())]))),
			syn::Fields::Unnamed(ref f) => out.extend(f.unnamed.iter().enumerate().map(|(i,f)| (f.ty.clone(),vec![FieldPos::Unnamed(i)]))),
			syn::Fields::Unit           => return out,
		}

		for variant in variants{
			out.retain_mut(|(ty,poss)|
				match find_field_by_type(&variant.fields,ty,names.iter().cloned().map(|(x,_)| x)){
					Some(pos) => {poss.push(pos); true},
					None => false
				}
			);
		}
	}
	out
}

fn gen_match_arms<'v,'i>(enum_ident: &syn::Ident,variants: impl Iterator<Item = (&'v syn::Variant,&'i FieldPos)>) -> impl Iterator<Item = TokenStream>{
	variants.map(move |(variant,pos)|{
		let variant_ident = &variant.ident;
		let var_ident = &syn::parse_quote!{ _y };
		let pat = pos.to_pat(&syn::parse_quote!{ #enum_ident::#variant_ident },var_ident);
		quote!{ #pat => #var_ident, }
	})
}

#[cfg(feature = "derive_into")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let impls = common_field_types(item.variants.iter());
	if impls.len() == 0{return syn::Error::new(item.span(),"`derive(EnumInto)` is applied to an enum without any common types shared by all its fields.").into_compile_error();}
	let impls = impls.into_iter().map(|(ty,poss)|{
		let match_arms = gen_match_arms(ident,item.variants.iter().zip(poss.iter()));
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

/*
#[cfg(feature = "attr_into")]
pub fn gen_attr_names(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util;
	use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_tokenstream_itemkind};
	use syn::punctuated::Punctuated;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let len = item.variants.len();

	let fns = util::try_tokenstream!(syn::parse2::<Punctuated<ItemPrefix<Concat<ItemKind,syn::Ident>>,Token![,]>>(attr));
	//let ItemPrefix(attrs,vis,Concat(ref kind,const_ident)) = ;
	//let kind = assert_tokenstream_itemkind!(kind,ItemKind::Fn(..),&ItemKind::r#fn());
	quote!{
		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fns )*
		}
	}
}
*/
