use proc_macro2::TokenStream;

fn gen_match_arms<'i>(ident: &syn::Ident,variants: impl Iterator<Item = &'i syn::Variant>) -> impl Iterator<Item = TokenStream>{
	use alloc::string::ToString as _;
	use syn::Fields;
	variants.map(move |variant| {
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
	})
}

#[cfg(feature = "derive_variant_name")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(ident,item.variants.iter());

	quote!{
		#[automatically_derived]
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

#[cfg(feature = "attr_variant_name")] use crate::util::parse::ItemPrefix;

#[cfg(feature = "attr_variant_name")]
pub fn gen_attr(
	ItemPrefix(fn_attrs,fn_vis,sign): ItemPrefix<syn::Signature>,
	item: syn::ItemEnum
) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(ident,item.variants.iter());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fn_attrs )* #fn_vis #sign{
				match self{
					#( #match_arms )*
				}
			}
		}
	}
}
