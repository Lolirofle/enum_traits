use crate::util::occurs;
use proc_macro2::TokenStream;
use syn::{Fields,Generics};
use syn::visit::Visit;

#[cfg(feature = "derive_field_structs")]
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
				let mut generics: Generics = item.generics.clone(); //TODO: Remove all where bounds that mention the removed vars
				generics.params = syn::punctuated::Punctuated::from_iter(generics.params.into_iter().filter(|param| match param{
					syn::GenericParam::Lifetime(syn::LifetimeParam{lifetime,..}) => {
						let mut occurs = occurs::LifetimeOccursVisit::new(lifetime);
						occurs.visit_fields_unnamed(fields);
						occurs.found
					},
					syn::GenericParam::Type(syn::TypeParam{ident,..}) => {
						let mut occurs = occurs::TypeOccursVisit::new(ident);
						occurs.visit_fields_unnamed(fields);
						occurs.0.found
					},
					syn::GenericParam::Const(syn::ConstParam{ident,..}) => {
						let mut occurs = occurs::IdentOccursVisit::new(ident);
						occurs.visit_fields_unnamed(fields);
						occurs.found
					},
				}));

				quote! {
					#[automatically_derived]
					#visibility struct #variant_ident #generics #fields;
				}
			},
			Fields::Named(ref fields) => {
				let mut generics: Generics = item.generics.clone(); //TODO: Remove all where bounds that mention the removed vars
				generics.params = syn::punctuated::Punctuated::from_iter(generics.params.into_iter().filter(|param| match param{
					syn::GenericParam::Lifetime(syn::LifetimeParam{lifetime,..}) => {
						let mut occurs = occurs::LifetimeOccursVisit::new(lifetime);
						occurs.visit_fields_named(fields);
						occurs.found
					},
					syn::GenericParam::Type(syn::TypeParam{ident,..}) => {
						let mut occurs = occurs::TypeOccursVisit::new(ident);
						occurs.visit_fields_named(fields);
						occurs.0.found
					},
					syn::GenericParam::Const(syn::ConstParam{ident,..}) => {
						let mut occurs = occurs::IdentOccursVisit::new(ident);
						occurs.visit_fields_named(fields);
						occurs.found
					},
				}));

				quote! {
					#[automatically_derived]
					#visibility struct #variant_ident #generics #fields
				}
			},
		}
	});

	quote!{
		#( #variant_structs )*
	}
}

//TODO: Attribute that transforms the original enum to have the structs as contents instead in addition to above
