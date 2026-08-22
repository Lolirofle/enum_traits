use crate::util;
use crate::util::parse::ItemPrefix;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_variants_array")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumVariantsArray)"));

	let contents = item.variants.iter().map(|variant|{
		let variant_ident = variant.ident;
		quote! { #ident::#variant_ident, }
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::VariantsArray for #ident #ty_generics #where_clause{
			const VARIANTS: [Self; <Self as ::enum_traits:::Len>::LEN] = [#( #contents )*];
		}
	}
}

#[cfg(feature = "attr_variants_array")]
pub fn gen_attr_impl(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"enum_variants_array"));

	let len = item.variants.len();
	let contents = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;
		quote! { #ident::#variant_ident, }
	});

	let ItemPrefix(attrs,vis,const_ident) = util::try_tokenstream!(syn::parse2::<ItemPrefix<syn::Ident>>(attr));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis const #const_ident: [Self; #len] = [#( #contents )*];
		}
	}
}
