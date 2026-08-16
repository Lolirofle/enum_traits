use proc_macro2::{Literal,TokenStream};
use syn::{Fields,Lit};

#[cfg(feature = "derive_to_index")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
		let variant_ident = &variant.ident;
		let i = Lit::new(Literal::usize_unsuffixed(i));

		match variant.fields{
			Fields::Unit => {
				quote! { &#ident::#variant_ident => #i, }
			}
			Fields::Unnamed(_) => {
				quote! { &#ident::#variant_ident(..) => #i, }
			}
			Fields::Named(_) => {
				quote! { &#ident::#variant_ident{..} => #i, }
			}
		}
	});

	let match_arms_into = item.variants.iter().enumerate().map(|(i,variant)|{
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
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::ToIndex for #ident #ty_generics #where_clause{
			fn into_index(self) -> <Self as ::enum_traits::Index>::Type{
				match self{
					#( #match_arms_into )*
				}
			}
			fn index(&self) -> <Self as ::enum_traits::Index>::Type{
				match self{
					#( #match_arms )*
					_ => unreachable!()
				}
			}
		}
	}
}
