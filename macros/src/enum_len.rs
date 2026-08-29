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

#[cfg(feature = "attr_len")] use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_tokenstream_itemkind};

#[cfg(feature = "attr_len")]
pub fn gen_attr(
	ItemPrefix(attrs,vis,Concat(ref kind,const_ident)): ItemPrefix<Concat<ItemKind,syn::Ident>>,
	item: syn::ItemEnum
) -> TokenStream{
	let kind = assert_tokenstream_itemkind!(kind,ItemKind::Const(..),&ItemKind::r#const());

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let len = item.variants.len();

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #const_ident: usize = #len;
		}
	}
}
