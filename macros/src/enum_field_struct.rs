use core::default::Default;
use crate::util;
use crate::util::occurs;
use crate::util::parse::ItemPrefix;
use proc_macro2::TokenStream;
use syn::{Fields,Generics};
use syn::visit::Visit as _;

fn generic_param_to_arg(param: &syn::GenericParam) -> syn::GenericArgument{match param{
	syn::GenericParam::Lifetime(p) => syn::GenericArgument::Lifetime(p.lifetime.clone()),
	syn::GenericParam::Type(p)     => {let i = &p.ident; syn::GenericArgument::Type(syn::parse_quote!{ #i })},
	syn::GenericParam::Const(p)    => {let i = &p.ident; syn::GenericArgument::Const(syn::parse_quote!{ #i })},
}}

fn generic_params_to_args(g: &syn::Generics) -> syn::AngleBracketedGenericArguments{syn::AngleBracketedGenericArguments{
	colon2_token: None,
	lt_token: g.lt_token.map_or_else(|| Default::default(),|t| t.clone()),
	gt_token: g.gt_token.map_or_else(|| Default::default(),|t| t.clone()),
	args: g.params.pairs()
		.map(|p| syn::punctuated::Pair::new(
			generic_param_to_arg(p.value()),
			p.punct().copied().copied()
		))
		.collect()
}}

fn gen_struct(variant: &syn::Variant,generics: &syn::Generics,ItemPrefix(attrs,vis,struct_ident): ItemPrefix<Option<syn::Ident>>) -> syn::ItemStruct{
	let struct_ident = struct_ident.unwrap_or_else(|| variant.ident.clone());

	match variant.fields {
		Fields::Unit => syn::parse_quote! {
			#[automatically_derived]
			#( #attrs )*
			#vis struct #struct_ident;
		},
		Fields::Unnamed(ref fields) => {
			let mut generics: Generics = generics.clone(); //TODO: Remove all where bounds that mention the removed vars
			generics.params = syn::punctuated::Punctuated::from_iter(generics.params.into_iter().filter(|param| match param{
				syn::GenericParam::Lifetime(syn::LifetimeParam{lifetime,..}) => {
					let mut occurs = occurs::OccursVisit::new(lifetime);
					occurs.visit_fields_unnamed(fields);
					occurs.found
				},
				syn::GenericParam::Type(syn::TypeParam{ident,..}) => {
					let mut occurs = occurs::TypeOccursVisit::new(ident);
					occurs.visit_fields_unnamed(fields);
					occurs.0.found
				},
				syn::GenericParam::Const(syn::ConstParam{ident,..}) => {
					let mut occurs = occurs::OccursVisit::new(ident);
					occurs.visit_fields_unnamed(fields);
					occurs.found
				},
			}));

			syn::parse_quote! {
				#[automatically_derived]
				#( #attrs )*
				#vis struct #struct_ident #generics #fields;
			}
		},
		Fields::Named(ref fields) => {
			let mut generics: Generics = generics.clone(); //TODO: Remove all where bounds that mention the removed vars
			generics.params = syn::punctuated::Punctuated::from_iter(generics.params.into_iter().filter(|param| match param{
				syn::GenericParam::Lifetime(syn::LifetimeParam{lifetime,..}) => {
					let mut occurs = occurs::OccursVisit::new(lifetime);
					occurs.visit_fields_named(fields);
					occurs.found
				},
				syn::GenericParam::Type(syn::TypeParam{ident,..}) => {
					let mut occurs = occurs::TypeOccursVisit::new(ident);
					occurs.visit_fields_named(fields);
					occurs.0.found
				},
				syn::GenericParam::Const(syn::ConstParam{ident,..}) => {
					let mut occurs = occurs::OccursVisit::new(ident);
					occurs.visit_fields_named(fields);
					occurs.found
				},
			}));

			syn::parse_quote! {
				#[automatically_derived]
				#( #attrs )*
				#vis struct #struct_ident #generics #fields
			}
		},
	}
}

#[cfg(feature = "derive_field_structs")]
pub fn gen_derive(mut item: syn::ItemEnum) -> TokenStream{
	let enum_ident = &item.ident;
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();

	let variant_structs = item.variants.iter_mut().map(|variant|{
		let params = util::try_tokenstream!(util::attr_params!(enum_field_struct,variant.attrs , exclude ; name: ItemPrefix<syn::Ident>));
		if params.exclude{
			return quote!{};
		}

		let item = gen_struct(
			variant,
			&item.generics,
			params.name.map_or(ItemPrefix(Default::default(),item.vis.clone(),None),|ItemPrefix(attrs,vis,ident)| ItemPrefix(attrs,vis,Some(ident)))
		);
		let expr = crate::enum_from::fields_to_expr(&variant);

		let struct_ident = &item.ident;
		let struct_generics = generic_params_to_args(&item.generics);

		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics ::core::convert::From<#struct_ident #struct_generics> for #enum_ident #ty_generics #where_clause{
				#[inline(always)]
				fn from(#expr: #struct_ident #struct_generics) -> Self{
					#enum_ident::#expr
				}
			}

			#[automatically_derived]
			impl #impl_generics ::core::convert::TryFrom<#enum_ident #ty_generics> for #struct_ident #struct_generics #where_clause{
				type Error = ();

				#[inline(always)]
				fn try_from(e: #enum_ident #ty_generics) -> ::core::result::Result<Self,Self::Error>{
					if let #enum_ident::#expr = e {::core::result::Result::Ok(#expr)} else {::core::result::Result::Err(())}
				}
			}
		}
	});

	quote!{
		#( #variant_structs )*
	}
}

#[cfg(feature = "attr_field_structs")]
pub fn gen_attr(_param: TokenStream,mut item: syn::ItemEnum) -> TokenStream{
	let variant_structs: TokenStream = item.variants.iter_mut().map(|variant|{
		let params = util::try_tokenstream!(util::attr_params!(enum_field_struct,variant.attrs , exclude ; name: ItemPrefix<syn::Ident>));
		if params.exclude{
			return quote!{};
		}

		let item = gen_struct(
			variant,
			&item.generics,
			params.name.map_or(ItemPrefix(Default::default(),item.vis.clone(),None),|ItemPrefix(attrs,vis,ident)| ItemPrefix(attrs,vis,Some(ident)))
		);

		let struct_ident = &item.ident;
		let struct_generics = generic_params_to_args(&item.generics);

		//Transform.
		variant.fields = syn::Fields::Unnamed(syn::parse_quote!{ (#struct_ident #struct_generics) });

		quote!{
			#item
		}
	}).collect();

	quote!{
		#item
		#variant_structs
	}
}
