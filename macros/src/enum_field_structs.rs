use crate::util::free_vars::FreeVarsVisit;
use proc_macro2::TokenStream;
use syn::{Fields,Generics};
use syn::visit::Visit;

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let ref visibility = item.vis;

	let variant_structs = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;

		match variant.fields {
			Fields::Unit => quote! {
				#[automatically_derived]
				#visibility struct #variant_ident;
			},
			Fields::Unnamed(ref fields) => {
				let mut fields_vars = FreeVarsVisit::new();
				fields_vars.visit_fields_unnamed(fields);

				let generics: Generics = item.generics.iter(); //TODO: Remove all where bounds that mention the removed vars
				let fields = fields.unnamed.iter();
				//let fields = fields.unnamed.iter().map(|field| field.ty);
				quote! {
					#[automatically_derived]
					#visibility struct #variant_ident(#( #fields ),*);
				}
			},
			Fields::Named(ref fields) => {
				let fields = fields.named.iter();
				quote! {
					#[automatically_derived]
					#visibility struct #variant_ident{#( #fields ),*}
				}
			},
		}
	});

	quote!{
		#( #variant_structs )*
	}
}
