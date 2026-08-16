use alloc::string::ToString;
use proc_macro2::TokenStream;
use syn::Fields;

#[cfg(feature = "derive_variant_name")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream {
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
