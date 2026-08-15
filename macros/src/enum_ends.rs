use proc_macro2::TokenStream;
use syn::Fields;

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;
	let variant_first = item.variants.first().expect("`derive(EnumEnds)` may only be applied to non-empty enums");
	let variant_last  = item.variants.last().expect("`derive(EnumEnds)` may only be applied to non-empty enums");
	if let Fields::Unit = variant_first.fields {} else {panic!("`derive(EnumEnds)` may only be applied to enums where the first variant is an unit variant");}
	if let Fields::Unit = variant_last.fields {} else {panic!("`derive(EnumEnds)` may only be applied to enums where the last variant is an unit variant");}
	let variant_first_ident = &variant_first.ident;
	let variant_last_ident  = &variant_last.ident;

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Ends for #ident #ty_generics #where_clause{
			const FIRST: Self = #ident::#variant_first_ident;
			const LAST : Self = #ident::#variant_last_ident;
		}
	}
}
