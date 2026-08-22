use crate::util;
use proc_macro2::TokenStream;

fn gen_prev_fn_body<'i>(ident: &syn::Ident,variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> TokenStream{
	let match_arms = {
		let iter = variants.clone().map(|v| &v.ident);
		iter.zip(variants.map(|v| &v.ident).skip(1))
	}.map(|(variant_ident1,variant_ident2)|{
		quote! { #ident::#variant_ident2 => #ident::#variant_ident1, }
	});

	quote!{
		::core::option::Option::Some(match self{
			#( #match_arms )*
			_ => return ::core::option::Option::None
		})
	}
}

fn gen_next_fn_body<'i>(ident: &syn::Ident,variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> TokenStream{
	let match_arms = {
		let iter = variants.clone().map(|v| &v.ident);
		iter.zip(variants.map(|v| &v.ident).skip(1))
	}.map(|(variant_ident1,variant_ident2)|{
		quote! { #ident::#variant_ident1 => #ident::#variant_ident2, }
	});

	quote!{
		::core::option::Option::Some(match self{
			#( #match_arms )*
			_ => return ::core::option::Option::None
		})
	}
}

#[cfg(feature = "derive_step")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumStep)"));

	let prev_body = gen_prev_fn_body(ident,item.variants.iter());
	let next_body = gen_next_fn_body(ident,item.variants.iter());
	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Step for #ident #ty_generics #where_clause{
			#[inline]
			#[allow(unreachable_code)]
			fn next(self) -> ::core::option::Option<Self>{
				#next_body
			}

			#[inline]
			#[allow(unreachable_code)]
			fn previous(self) -> ::core::option::Option<Self>{
				#prev_body
			}
		}
	}
}

#[cfg(feature = "attr_step")]
pub fn gen_attr_prev(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util::parse::ItemPrefix;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let body = gen_prev_fn_body(ident,item.variants.iter());

	let ItemPrefix(attrs,vis,sign) = util::try_tokenstream!(syn::parse2::<ItemPrefix<syn::Signature>>(attr));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #sign{#body}
		}
	}
}

#[cfg(feature = "attr_step")]
pub fn gen_attr_next(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util::parse::ItemPrefix;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let body = gen_next_fn_body(ident,item.variants.iter());

	let ItemPrefix(attrs,vis,sign) = util::try_tokenstream!(syn::parse2::<ItemPrefix<syn::Signature>>(attr));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #sign{#body}
		}
	}
}
