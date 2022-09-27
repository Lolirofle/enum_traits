use proc_macro2::TokenStream;
use syn::Fields;

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;
	let ref visibility = item.vis;

	let unit_enum_ident = format_ident!("{}Tag",ident);

	let match_arms = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;

		match variant.fields {
			Fields::Unit => {
				quote! { &#ident::#variant_ident     => #unit_enum_ident::#variant_ident, }
			}
			Fields::Unnamed(_) => {
				quote! { &#ident::#variant_ident(..) => #unit_enum_ident::#variant_ident, }
			}
			Fields::Named(_) => {
				quote! { &#ident::#variant_ident{..} => #unit_enum_ident::#variant_ident, }
			}
		}
	});

	let match_arms_into = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;

		match variant.fields {
			Fields::Unit => {
				quote! { #ident::#variant_ident     => #unit_enum_ident::#variant_ident, }
			}
			Fields::Unnamed(_) => {
				quote! { #ident::#variant_ident(..) => #unit_enum_ident::#variant_ident, }
			}
			Fields::Named(_) => {
				quote! { #ident::#variant_ident{..} => #unit_enum_ident::#variant_ident, }
			}
		}
	});

	let unit_variants = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;
		quote! { #variant_ident, }
	});

	quote!{
		#[automatically_derived]
		#[allow(unused_attributes)]
		#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
		#visibility enum #unit_enum_ident{
			#( #unit_variants )*
		}

		#[automatically_derived]
		#[allow(unused_attributes)]
		impl #impl_generics ::enum_traits::Tag for #ident #ty_generics #where_clause{
			type Tag = #unit_enum_ident;

			#[inline]
			fn into_tag(self) -> Self::Tag{
				match self{
					#( #match_arms_into )*
				}
			}

			#[inline]
			fn tag(&self) -> Self::Tag{
				match self{
					#( #match_arms )*
				}
			}
		}
	}
}
