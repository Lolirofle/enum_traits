use proc_macro2::TokenStream;

#[cfg(feature = "derive_len")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;
	let len = item.variants.len();

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Len for #ident #ty_generics #where_clause{
			const LEN: usize = #len;
		}
	}
}

#[cfg(feature = "attr_len")]
pub fn gen_attr(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util;
	use crate::util::parse::ItemPrefix;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let len = item.variants.len();

	let ItemPrefix(attrs,vis,const_ident) = util::try_tokenstream!(syn::parse2::<ItemPrefix<syn::Ident>>(attr));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis const #const_ident: usize = #len;
		}
	}
}
