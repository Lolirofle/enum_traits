//<Self as ::core::marker::DiscriminantKind>::Discriminant
use crate::util;
use proc_macro2::{Span,TokenStream};
use syn::{Ident,TypeParam,Variant,parse_quote};

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let type_param = &Ident::new("__EnumTraitsFromDiscriminantParam",Span::mixed_site());
	let generics_added = {
		let mut generics = item.generics.clone();

		let where_clause = generics.make_where_clause();
		where_clause.predicates.push(parse_quote!{ Self: ::enum_traits::IntoDiscriminant<#type_param> });
		where_clause.predicates.push(parse_quote!{ #type_param: ::core::cmp::PartialEq });

		let type_param: TypeParam = type_param.clone().into();
		generics.params.push(type_param.into());

		generics
	};
	let (impl_generics,_,where_clause) = generics_added.split_for_impl();
	let (_,ty_generics,_) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &Ident,variant: &Variant,type_param: &Ident) -> TokenStream{
		let variant_ident = util::variant_unit_ident(&variant,"EnumFromDiscriminant");
		quote! { n if n == <Self as ::enum_traits::IntoDiscriminant<#type_param>>::into_discriminant(#ident::#variant_ident) => #ident::#variant_ident }
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromDiscriminant<#type_param> for #ident #ty_generics #where_clause{
			#[inline]
			fn from_discriminant(discriminant: #type_param) -> Option<Self>{
				Some(match discriminant{
					#( #match_arms1 , )*
					_ => return None
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

/*
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	if item.generics.type_params().next().is_some()
	|| item.generics.lifetimes().next().is_some()
	|| item.generics.const_params().next().is_some()
	{
		panic!("EnumFromDiscriminant on enums with type parameters or where clauses are unimplemented.");
	}
	let type_param = &Ident::new("_EnumTraitsFromDiscriminantParam",Span::mixed_site());
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &Ident,variant: &Variant,type_param: &Ident) -> TokenStream{
		let variant_ident = util::variant_unit_ident(&variant,"EnumFromDiscriminant");
		quote! { n if n == <Self as ::enum_traits::IntoDiscriminant<#type_param>>::into_discriminant(#ident::#variant_ident) => #ident::#variant_ident }
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant,type_param));

	quote!{
		#[automatically_derived]
		impl<#type_param> ::enum_traits::FromDiscriminant<#type_param> for #ident where
			Self: ::enum_traits::IntoDiscriminant<#type_param>,
			#type_param: ::core::cmp::PartialEq,
		{
			#[inline]
			fn from_discriminant(discriminant: #type_param) -> Option<Self>{
				Some(match discriminant{
					#( #match_arms1 , )*
					_ => return None
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
*/

/*
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &Ident,variant: &Variant) -> TokenStream{
		let variant_ident = util::variant_unit_ident(&variant,"EnumFromDiscriminant");
		quote! { n if n == ::core::intrinsics::discriminant_value(&#ident::#variant_ident) => #ident::#variant_ident }
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromDiscriminant<<Self as ::core::marker::DiscriminantKind>::Discriminant> for #ident #ty_generics #where_clause{
			#[inline]
			fn from_discriminant(discriminant: <Self as ::core::marker::DiscriminantKind>::Discriminant) -> Option<Self>{
				Some(match discriminant{
					#( #match_arms1 , )*
					_ => return None
				})
			}

			#[inline]
			unsafe fn from_discriminant_unchecked(discriminant: <Self as ::core::marker::DiscriminantKind>::Discriminant) -> Self{
				match discriminant{
					#( #match_arms2 , )*
					_ => ::core::hint::unreachable_unchecked()
				}
			}
		}
	}
}
*/

/*
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let type_param = &Ident::new("T",Span::mixed_site());
	let generics_added = {
		let mut generics = item.generics.clone();
		let mut type_param: syn::TypeParam = type_param.clone().into();
		type_param.colon_token = Some(syn::token::Colon::default());
		type_param.bounds.push(syn::TraitBound{
			paren_token: None,
			modifier: syn::TraitBoundModifier::None,
			lifetimes: None,
			path: util::path_segments_to_path(true,[
				syn::PathSegment{
					ident: Ident::new("enum_traits",Span::call_site()),
					arguments: syn::PathArguments::None
				},
				syn::PathSegment{
					ident: Ident::new("IntoDiscriminant",Span::call_site()),
					arguments: syn::PathArguments::AngleBracketed()
				},
			]),
		}.into());
		generics.params.push(type_param.into());
		generics
	};
	let (impl_generics,_,_) = generics_added.split_for_impl();
	let (_,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &Ident,variant: &Variant) -> TokenStream{
		let variant_ident = util::variant_unit_ident(&variant,"EnumFromDiscriminant");
		quote! { n if n == ::enum_traits::IntoDiscriminant::into_discriminant(#ident::#variant_ident) => #ident::#variant_ident }
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromDiscriminant<#type_param> for #ident #ty_generics #where_clause{
			#[inline]
			fn from_discriminant(discriminant: #type_param) -> Option<Self>{
				Some(match discriminant{
					#( #match_arms1 , )*
					_ => return None
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
*/

/*
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let type_param = &Ident::new("T",Span::mixed_site());
	let generics_added = {
		let mut generics = item.generics.clone();
		let mut type_param: syn::TypeParam = type_param.clone().into();
		type_param.colon_token = Some(syn::token::Colon::default());
		type_param.bounds.push(syn::TraitBound{
			paren_token: None,
			modifier: syn::TraitBoundModifier::None,
			lifetimes: None,
			path: util::idents_to_path(&[
				Ident::new("enum_traits",Span::call_site()),
				Ident::new("IntoDiscriminant",Span::call_site()),
			],true),
		}.into());
		generics.params.push(type_param.into());
		generics
	};
	let (impl_generics,_,_) = generics_added.split_for_impl();
	let (_,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn variant_to_match_arm(ident: &Ident,variant: &Variant) -> TokenStream{
		let variant_ident = util::variant_unit_ident(&variant,"EnumFromDiscriminant");

		//Whether an explicit discriminant exist
		variant.discriminant.as_ref().map(|(_,ref variant_discriminant)|
			quote! { #variant_discriminant => #ident::#variant_ident }
		).unwrap_or_else(||
			quote! { n if n == ::enum_trait::IntoDiscriminant::into_discriminant(#ident::#variant_ident) => #ident::#variant_ident }
		)
	}

	let match_arms1 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));
	let match_arms2 = item.variants.iter().map(|variant| variant_to_match_arm(ident,variant));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromDiscriminant<#type_param> for #ident #ty_generics #where_clause{
			#[inline]
			fn from_discriminant(discriminant: #type_param) -> Option<Self>{
				Some(match discriminant{
					#( #match_arms1 , )*
					_ => return None
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
*/

/*
let only_unit_variants = item.variants.iter().all(|variant| match variant.fields{Fields::Unit => true , _ => false});
let into_fn = if only_unit_variants{
	quote!{ self as #ty }
}else{

}

#[inline(always)]
fn into_discriminant(self) -> #ty{
	self as #ty
}
 */
