use alloc::string::ToString;
use proc_macro2::TokenStream;
use syn::Fields;

#[cfg(feature = "derive_from_str")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let match_arms = item.variants.iter().filter_map(|variant| {
		let variant_ident = &variant.ident;
		let variant_str = variant.ident.to_string();

		if let Fields::Unit = variant.fields{
			Some(quote! { #variant_str => #ident::#variant_ident, })
		}else{
			None
		}
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics ::core::str::FromStr for #ident #ty_generics #where_clause{
			type Err = ();

			fn from_str(str: &str) -> ::core::result::Result<Self,Self::Err>{
				::core::result::Result::Ok(match str{
					#( #match_arms )*
					_ => return Err(())
				})
			}
		}
	}
}
