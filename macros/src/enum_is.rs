use crate::util;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_is")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	use alloc::string::ToString as _;
	use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_tokenstream_itemkind};
	use syn::ext::IdentExt as _;
	use syn::Fields;
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;
	let fn_attrs = quote!( #[inline(always)] #[allow(dead_code)] );

	//Generation

	let fns = item.variants.iter().map(|variant|{
		let params = util::try_tokenstream!(util::attr_params!(enum_is,variant.attrs , exclude ; name: ItemPrefix<Concat<ItemKind,syn::Ident>>));
		if params.exclude{
			return quote!{};
		}

		let (fn_attrs,fn_vis,fn_kind,fn_ident) = match params.name{
			Some(ItemPrefix(attrs,ref vis,Concat(kind,ident))) => {
				let kind = assert_tokenstream_itemkind!(kind,ItemKind::Fn(..),ItemKind::const_fn());
				(&quote!( #( #attrs )* ),vis,kind,ident)
			},
			None => (&fn_attrs,&item.vis,ItemKind::const_fn(),format_ident!("is_{}",util::camelcase_to_snakecase(variant.ident.unraw().to_string().as_ref())))
		};

		let pattern = {
			let variant_ident = &variant.ident;
			match variant.fields{
				Fields::Unit       => quote! { #ident::#variant_ident },
				Fields::Unnamed(_) => quote! { #ident::#variant_ident(..) },
				Fields::Named(_)   => quote! { #ident::#variant_ident{..} },
			}
		};

		quote! {
			#fn_attrs #fn_vis #fn_kind #fn_ident(&self) -> bool{
				if let #pattern = self{true}else{false}
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
