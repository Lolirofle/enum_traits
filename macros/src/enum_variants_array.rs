use crate::util;
use crate::util::ident_attr_vis::IdentAttrVis;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_variants_array")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let contents = item.variants.iter().map(|variant|{
		let variant_ident = util::variant_unit_ident(&variant,"EnumVariantsArray");
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

	let len = item.variants.len();
	let contents = item.variants.iter().map(|variant|{
		let variant_ident = util::variant_unit_ident(&variant,"impl_enum_variants_array");
		quote! { #ident::#variant_ident, }
	});

	let IdentAttrVis{attrs,vis,ident: const_ident} = syn::parse2::<IdentAttrVis>(attr)
		.expect("`impl_enum_variants_array` expects an ident argument.");

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis const #const_ident: [Self; #len] = [#( #contents )*];
		}
	}
}
