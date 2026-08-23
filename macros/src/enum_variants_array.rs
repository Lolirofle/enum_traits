use crate::util;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_variants_array")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumVariantsArray)"));

	let contents = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;
		quote! { #ident::#variant_ident, }
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::VariantsArray for #ident #ty_generics #where_clause{
			const VARIANTS: &'static [Self] = &[#( #contents )*];
		}
	}
}

#[cfg(feature = "attr_variants_array")]
pub fn gen_attr(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_tokenstream_itemkind};

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"enum_variants_array"));

	let len = item.variants.len();
	let contents = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;
		quote! { #ident::#variant_ident, }
	});

	let ItemPrefix(attrs,vis,Concat(ref kind,const_ident)) = util::try_tokenstream!(syn::parse2::<ItemPrefix<Concat<ItemKind,syn::Ident>>>(attr));
	let kind = assert_tokenstream_itemkind!(kind,ItemKind::Const(..),&ItemKind::r#const());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #const_ident: [Self; #len] = [#( #contents )*];
		}
	}
}
