use proc_macro2::TokenStream;
use syn::{Fields,FieldsNamed,FieldsUnnamed,Index,Member};

fn fields_to_ty(fields: &syn::Fields) -> TokenStream{
	match fields{
		Fields::Unit => quote! { () },
		Fields::Unnamed(FieldsUnnamed{unnamed: fields,..}) |
		Fields::Named(FieldsNamed{named: fields,..}) => {
			match fields.len(){
				1 => {
					let ty = &fields.first().unwrap().ty;
					quote! { #ty }
				},
				_ => {
					let tys = fields.iter().map(|field| &field.ty);
					quote! { (#(#tys),*) }
				},
			}
		}
	}
}

fn fields_to_pat(fields: &syn::Fields) -> TokenStream{
	match fields{
		Fields::Unit => quote! { () },
		Fields::Unnamed(FieldsUnnamed{unnamed: fields,..}) |
		Fields::Named(FieldsNamed{named: fields,..}) => {
			match fields.len(){
				1 => quote! { x0 },
				_ => {
					let vars = fields.iter()
						.enumerate()
						.map(|(i,_field)| format_ident!("x{}",i));
					quote! { ( #(#vars),* ) }
				},
			}
		}
	}
}

pub fn fields_to_expr(variant_ident: &syn::Ident,fields: &syn::Fields) -> TokenStream{
	match fields{
		Fields::Unit => quote! { #variant_ident },
		Fields::Unnamed(FieldsUnnamed{unnamed: fields,..}) => {
			let args = fields.iter()
				.enumerate()
				.map(|(i,_field)| format_ident!("x{}",i));
			quote! { #variant_ident( #(#args),* ) }
		},
		Fields::Named(FieldsNamed{named: fields,..}) => {
			let args = fields.iter()
				.enumerate()
				.map(|(i,_field)| format_ident!("x{}",i));
			let field_names = fields.iter()
				.enumerate()
				.map(|(i,field)| match field.ident{
					Some(ref ident) => Member::Named(ident.clone()),
					None => Member::Unnamed(Index::from(i)),
				});
			quote! { #variant_ident{ #(#field_names : #args),* } }
		},
	}
}

#[cfg(feature = "derive_from")]
pub fn gen_derive(mut item: syn::ItemEnum) -> TokenStream{
	use crate::util;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let impls = item.variants.iter_mut().filter_map(|variant|{
		//Blacklist specific variants (TODO: or the kinds of fields? Whitelist?)
		{
			let params = util::try_tokenstream!(util::attr_params!(enum_from,variant.attrs,exclude;));
			if params.exclude{
				return None;
			}
		}

		let ty = fields_to_ty(&variant.fields);
		let pattern = fields_to_pat(&variant.fields);
		let expr = fields_to_expr(&variant.ident,&variant.fields);
		Some(quote!{
			#[automatically_derived]
			impl #impl_generics ::core::convert::From<#ty> for #ident #ty_generics #where_clause{
				#[inline(always)]
				fn from(#pattern: #ty) -> Self{
					#ident::#expr
				}
			}
		})
	});

	quote!{#( #impls )*}
}
