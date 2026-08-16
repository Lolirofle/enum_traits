use crate::util;
use core::cmp;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_index")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	//Determine which type to use from the number of variants.
	let ty = util::minimum_type_from_value(cmp::max(item.variants.len(),1)-1);

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Index for #ident #ty_generics #where_clause{
			type Type = #ty;
		}
	}
}
