use proc_macro2::TokenStream;

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{//TODO: Consider allowing structs. Number of variants of struct is always 1
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

pub fn gen_attr_impl(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let len = item.variants.len();

	let const_ident = syn::parse2::<syn::Ident>(attr)
		.expect("`impl_enum_len` expects an ident argument.");

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			const #const_ident: usize = #len;
		}
	}
}
