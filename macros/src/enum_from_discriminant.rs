use crate::util;
use proc_macro2::{Span,TokenStream};

#[cfg(feature = "derive_from_discriminant")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumFromDiscriminant)"));

	let type_param = &syn::Ident::new("D",Span::mixed_site());
	let generics_added = {
		let mut generics = item.generics.clone();

		let where_clause = generics.make_where_clause();
		where_clause.predicates.push(syn::parse_quote!{ Self: ::enum_traits::IntoDiscriminant<#type_param> });
		where_clause.predicates.push(syn::parse_quote!{ #type_param: ::core::cmp::PartialEq });

		let type_param: syn::TypeParam = type_param.clone().into();
		generics.params.push(syn::GenericParam::Type(type_param));

		generics
	};
	let (impl_generics,_,where_clause) = generics_added.split_for_impl();
	let (_,ty_generics,_) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &syn::Ident,variant: &syn::Variant,type_param: &syn::Ident) -> TokenStream{
		let variant_ident = &variant.ident;
		quote! { n if n == <Self as ::enum_traits::IntoDiscriminant<#type_param>>::into_discriminant(#ident::#variant_ident) => #ident::#variant_ident }
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromDiscriminant<#type_param> for #ident #ty_generics #where_clause{
			#[inline]
			fn from_discriminant(discriminant: #type_param) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match discriminant{
					#( #match_arms1 , )*
					_ => return ::core::option::Option::None
				})
			}

			#[inline]
			unsafe fn from_discriminant_unchecked(discriminant: #type_param) -> Self{
				match discriminant{
					#( #match_arms2 , )*
					_ => ::core::hint::unreachable_unchecked()
				}
			}
		}
	}
}
