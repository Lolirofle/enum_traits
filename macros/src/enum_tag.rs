use crate::util;
use crate::util::parse::{ItemPrefix,ItemKind};
use proc_macro2::TokenStream;
use syn::Fields;

#[cfg(feature = "derive_tag")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	//Attribute options
	let [ItemPrefix(unit_enum_attrs,unit_enum_vis,unit_enum_ident)]
		= util::try_tokenstream!(util::parse_itemprefix_attributes("enum_tag",[ItemKind::Enum],&item.attrs));
	let unit_enum_attrs = if unit_enum_attrs.is_empty(){
		quote!(#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)])
	}else{
		quote!( #( #unit_enum_attrs )* )
	};
	let unit_enum_vis = unit_enum_vis.unwrap_or(item.vis);
	let unit_enum_ident = unit_enum_ident.unwrap_or_else(|| format_ident!("{}Tag",ident));

	//Generation

	let match_arms = item.variants.iter().map(|variant|{
		let variant_ident = &variant.ident;
		match variant.fields{
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
		match variant.fields{
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
		#unit_enum_attrs
		#unit_enum_vis enum #unit_enum_ident{
			#( #unit_variants )*
		}

		#[automatically_derived]
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
