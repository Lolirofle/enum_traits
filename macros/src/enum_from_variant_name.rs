use alloc::string::ToString;
use proc_macro2::TokenStream;
use syn::Fields;

fn gen_match_arms<'v>(variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	variants.filter_map(|variant| {
		let variant_ident = &variant.ident;
		let variant_str = variant.ident.to_string();

		if let Fields::Unit = variant.fields{
			Some(quote! { #variant_str => Self::#variant_ident, })
		}else{
			None
		}
	})
}

#[cfg(feature = "derive_from_variant_name")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics ::core::str::FromStr for #ident #ty_generics #where_clause{
			type Err = ();

			fn from_str(str: &str) -> ::core::result::Result<Self,Self::Err>{
				::core::result::Result::Ok(match str{
					#( #match_arms )*
					_ => return Err(())
				})
			}
		}
	}
}

#[cfg(feature = "attr_from_variant_name")]
pub fn gen_attr(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util;
	use crate::util::parse::{ItemKind,ItemPrefix,Successive};

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let ItemPrefix(attrs,vis,Successive((item_kind,fn_ident))) = util::try_tokenstream!(syn::parse2::<ItemPrefix<Successive<(ItemKind,syn::Ident)>>>(attr));
	let item_kind = item_kind.or(ItemKind::default_fn());
	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #item_kind #fn_ident(str: &str) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match str{
					#( #match_arms )*
					_ => return None
				})
			}
		}
	}
}
