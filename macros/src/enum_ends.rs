use crate::util::ident_attr_vis::IdentAttrVis;
use proc_macro2::TokenStream;
use syn::Fields;

fn enum_first_variant_ident(item: &syn::ItemEnum) -> &syn::Ident{
	let variant_first = item.variants.first().expect("`impl_enum_first` may only be applied to non-empty enums");
	if let Fields::Unit = variant_first.fields {} else {panic!("`impl_enum_first` may only be applied to enums where the first variant is an unit variant");}
	&variant_first.ident
}

fn enum_last_variant_ident(item: &syn::ItemEnum) -> &syn::Ident{
	let variant_last = item.variants.last().expect("`impl_enum_last` may only be applied to non-empty enums");
	if let Fields::Unit = variant_last.fields {} else {panic!("`impl_enum_last` may only be applied to enums where the last variant is an unit variant");}
	&variant_last.ident
}

#[cfg(feature = "derive_ends")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_first_ident = enum_first_variant_ident(&item);
	let variant_last_ident  = enum_last_variant_ident(&item);

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Ends for #ident #ty_generics #where_clause{
			const FIRST: Self = #ident::#variant_first_ident;
			const LAST : Self = #ident::#variant_last_ident;
		}
	}
}

#[cfg(feature = "attr_ends")]
pub fn gen_attr_impl_first(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_first_ident = enum_first_variant_ident(&item);

	let IdentAttrVis{attrs,vis,ident: const_ident} = syn::parse2::<IdentAttrVis>(attr)
		.expect("`impl_enum_first` expects an ident argument.");

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis const #const_ident: Self = #ident::#variant_first_ident;
		}
	}
}

#[cfg(feature = "attr_ends")]
pub fn gen_attr_impl_last(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_last_ident = enum_last_variant_ident(&item);

	let IdentAttrVis{attrs,vis,ident: const_ident} = syn::parse2::<IdentAttrVis>(attr)
		.expect("`impl_enum_last` expects an ident argument.");

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis const #const_ident: Self = #ident::#variant_last_ident;
		}
	}
}
