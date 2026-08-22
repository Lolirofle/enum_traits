use crate::util;
use proc_macro2::{Literal,TokenStream};

#[cfg(feature = "derive_from_index")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn match_arm_transform(ident: &syn::Ident,(i,variant): (usize,&syn::Variant)) -> TokenStream{
		let i = syn::Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = &variant.ident;
		quote! { #i => #ident::#variant_ident, }
	}

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumFromIndex)"));

	let match_arms1 = item.variants.iter().enumerate().map(|arg| match_arm_transform(ident,arg));
	let match_arms2 = item.variants.iter().enumerate().map(|arg| match_arm_transform(ident,arg));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromIndex for #ident #ty_generics #where_clause{
			#[inline]
			fn from_index(index: <Self as ::enum_traits::Index>::Type) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match index{
					#( #match_arms1 )*
					_ => return ::core::option::Option::None
				})
			}

			#[inline]
			unsafe fn from_index_unchecked(index: <Self as ::enum_traits::Index>::Type) -> Self{
				match index{
					#( #match_arms2 )*
					_ => unreachable!()
				}
			}
		}
	}
}
