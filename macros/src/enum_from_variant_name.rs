use proc_macro2::TokenStream;

fn gen_match_arms<'v>(variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	use alloc::string::ToString as _;
	variants.filter_map(|variant| {
		let variant_ident = &variant.ident;
		let variant_str = variant.ident.to_string();

		if let syn::Fields::Unit = variant.fields{
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
					_ => return ::core::result::Result::Err(())
				})
			}
		}
	}
}

#[cfg(feature = "attr_from_variant_name")] use crate::util::parse::{Concat,Delimited,ItemKind,ItemPrefix};

#[cfg(feature = "attr_from_variant_name")]
pub fn gen_attr(
	ItemPrefix(attrs,vis,Concat(kind,fn_ident)): ItemPrefix<Concat<ItemKind,syn::Ident>>,
	item: syn::ItemEnum
) -> TokenStream{
	let kind = kind.or(ItemKind::r#fn());

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #fn_ident(str: &str) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match str{
					#( #match_arms )*
					_ => return ::core::option::Option::None
				})
			}
		}
	}
}

#[cfg(feature = "attr_from_variant_name")]
pub fn gen_attr_default(
	Delimited((ItemPrefix(attrs,vis,Concat(kind,fn_ident)),default)): Delimited<(ItemPrefix<Concat<ItemKind,syn::Ident>>,syn::Expr)>,
	item: syn::ItemEnum
) -> TokenStream{
	let kind = kind.or(ItemKind::r#fn());

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #fn_ident(__str: &str) -> Self{
				match __str{
					#( #match_arms )*
					_ => #default
				}
			}
		}
	}
}
