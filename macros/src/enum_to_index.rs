use proc_macro2::{Literal,TokenStream};

fn gen_match_arms<'i>(ident: &syn::Ident,variants: impl Iterator<Item = &'i syn::Variant>) -> impl Iterator<Item = TokenStream>{
	use syn::{Fields,Lit};
	variants.enumerate().map(move |(i,variant)|{
		let variant_ident = &variant.ident;
		let i = Lit::new(Literal::usize_unsuffixed(i));

		match variant.fields{
			Fields::Unit => {
				quote! { #ident::#variant_ident => #i, }
			}
			Fields::Unnamed(_) => {
				quote! { #ident::#variant_ident(..) => #i, }
			}
			Fields::Named(_) => {
				quote! { #ident::#variant_ident{..} => #i, }
			}
		}
	})
}

#[cfg(feature = "derive_to_index")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms_to = gen_match_arms(ident,item.variants.iter());
	let match_arms_into = gen_match_arms(ident,item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::EnumToIndex for #ident #ty_generics #where_clause{
			fn into_index(self) -> <Self as ::enum_traits::EnumIndex>::Type{
				match self{
					#( #match_arms_into )*
				}
			}
			fn index(&self) -> <Self as ::enum_traits::EnumIndex>::Type{
				match self{
					#( #match_arms_to )*
					_ => unreachable!()
				}
			}
		}
	}
}

#[cfg(feature = "attr_to_index")] use crate::util::parse::ItemPrefix;

#[cfg(feature = "attr_to_index")]
pub fn gen_attr(
	ItemPrefix(fn_attrs,fn_vis,fn_sign): ItemPrefix<syn::Signature>,
	item: syn::ItemEnum
) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(ident,item.variants.iter());

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fn_attrs )* #fn_vis #fn_sign{
				match self{
					#( #match_arms )*
				}
			}
		}
	}
}
