use alloc::string::ToString;
use crate::util;
use proc_macro2::TokenStream;
use syn::Fields;

#[cfg(feature = "derive_is")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let fns = item.variants.iter().map(|variant|{
		let fn_ident = format_ident!("is_{}",util::camelcase_to_snakecase(variant.ident.to_string().as_ref()));
		//TODO: Custom name using an attribute?

		let pattern = {
			let variant_ident = &variant.ident;
			match variant.fields{
				Fields::Unit => quote! { #ident::#variant_ident },
				Fields::Unnamed(_) => quote! { #ident::#variant_ident(..) },
				Fields::Named(_) => quote! { #ident::#variant_ident{..} },
			}
		};

		quote! {
			#[inline(always)]
			#[allow(dead_code)]
			pub const fn #fn_ident(&self) -> bool{
				if let &#pattern = self{true}else{false}
			}
		}
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fns )*
		}
	}
}
