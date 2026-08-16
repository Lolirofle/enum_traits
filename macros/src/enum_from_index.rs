use crate::util;
use proc_macro2::{Literal,TokenStream};
use syn::{Ident,Lit,Variant};

fn variant_unit_ident(variant: &Variant) -> &Ident{
	util::variant_unit_ident(variant,"EnumFromIndex")
}

#[cfg(feature = "derive_from_index")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	fn match_arm_transform(ident: &Ident,(i,variant_ident): (usize,&Ident)) -> TokenStream{
		let i = Lit::new(Literal::usize_unsuffixed(i));
		quote! { #i => #ident::#variant_ident, }
	}
	let match_arms1 = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));
	let match_arms2 = item.variants.iter().map(variant_unit_ident).enumerate().map(|arg| match_arm_transform(ident,arg));

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
