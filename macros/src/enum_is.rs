use alloc::string::ToString;
use crate::util;
use crate::util::parse::{ItemPrefix,ItemKind};
use proc_macro2::TokenStream;
use syn::Fields;

#[cfg(feature = "derive_is")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	//Attribute options
	let [ItemPrefix(fn_attrs,fn_vis,fn_ident_prefix)]
		= util::try_tokenstream!(util::parse_itemprefix_attributes("enum_is",[ItemKind::Fn],&item.attrs));
	let fn_attrs = if fn_attrs.is_empty(){
		quote!( #[inline(always)] #[allow(dead_code)] )
	}else{
		quote!( #( #fn_attrs )* )
	};
	let fn_vis = fn_vis.unwrap_or(item.vis);
	let fn_ident_prefix = fn_ident_prefix.unwrap_or_else(|| format_ident!("is_"));

	//Generation

	let fns = item.variants.iter().map(|variant|{
		let fn_ident = format_ident!("{}{}",fn_ident_prefix,util::camelcase_to_snakecase(variant.ident.to_string().as_ref()));
		//TODO: Custom name using an attribute?

		let pattern = {
			let variant_ident = &variant.ident;
			match variant.fields{
				Fields::Unit => quote! { #ident::#variant_ident },
				Fields::Unnamed(_) => quote! { #ident::#variant_ident(..) },
				Fields::Named(_) => quote! { #ident::#variant_ident{..} },
			}
		};

		quote! {
			#fn_attrs #fn_vis const fn #fn_ident(&self) -> bool{
				if let &#pattern = self{true}else{false}
			}
		}
	});

	quote!{
		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #fns )*
		}
	}
}
