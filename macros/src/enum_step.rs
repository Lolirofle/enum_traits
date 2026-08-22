use crate::util;
use proc_macro2::TokenStream;

#[cfg(feature = "derive_step")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumStep)"));

	let prev_match_arms = {
			let iter = item.variants.iter().rev().map(|v| &v.ident);
			iter.zip(item.variants.iter().rev().map(|v| &v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { #ident::#variant_ident1 => #ident::#variant_ident2, }
		});

	let next_match_arms = {
			let iter = item.variants.iter().map(|v| &v.ident);
			iter.zip(item.variants.iter().map(|v| &v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { #ident::#variant_ident1 => #ident::#variant_ident2, }
		});

	{
		let fn_next = quote!{
			#[inline]
			#[allow(unreachable_code)]
			fn next(self) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match self{
					#( #next_match_arms )*
					_ => return ::core::option::Option::None
				})
			}
		};

		let fn_prev = quote!{
			#[inline]
			#[allow(unreachable_code)]
			fn previous(self) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match self{
					#( #prev_match_arms )*
					_ => return ::core::option::Option::None
				})
			}
		};

		quote!{
			#[automatically_derived]
			impl #impl_generics ::enum_traits::Step for #ident #ty_generics #where_clause{
				#fn_next
				#fn_prev
			}
		}
	}
}
