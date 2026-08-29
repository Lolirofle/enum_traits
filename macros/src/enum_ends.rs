use core::fmt;
use crate::util;
use proc_macro2::TokenStream;
use syn::spanned::Spanned as _;

fn enum_first_variant_ident<'i>(item: &'i syn::ItemEnum,error_ctx: &'static str) -> syn::Result<&'i syn::Ident>{
	let variant_first = item.variants.first().ok_or_else(|| syn::Error::new(item.span(),"`impl_enum_first` may only be applied to non-empty enums"))?;
	util::check_unit_variant(variant_first,fmt::from_fn(|f| write!(f,"`{}` may only be applied to enums where the first variant is a unit variant",error_ctx)))?;
	Ok(&variant_first.ident)
}

fn enum_last_variant_ident<'i>(item: &'i syn::ItemEnum,error_ctx: &'static str) -> syn::Result<&'i syn::Ident>{
	let variant_last = item.variants.last().ok_or_else(|| syn::Error::new(item.span(),"`impl_enum_last` may only be applied to non-empty enums"))?;
	util::check_unit_variant(variant_last,fmt::from_fn(|f| write!(f,"`{}` may only be applied to enums where the last variant is a unit variant",error_ctx)))?;
	Ok(&variant_last.ident)
}

#[cfg(feature = "derive_ends")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_first_ident = util::try_tokenstream!(enum_first_variant_ident(&item,"derive(EnumEnds)"));
	let variant_last_ident  = util::try_tokenstream!(enum_last_variant_ident(&item,"derive(EnumEnds)"));

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::EnumEnds for #ident #ty_generics #where_clause{
			const FIRST: Self = #ident::#variant_first_ident;
			const LAST : Self = #ident::#variant_last_ident;
		}
	}
}

#[cfg(feature = "attr_ends")]
use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_tokenstream_itemkind};

#[cfg(feature = "attr_ends")]
pub fn gen_attr_first(
	ItemPrefix(attrs,vis,Concat((ref kind,const_ident))): ItemPrefix<Concat<(ItemKind,syn::Ident)>>,
	item: syn::ItemEnum
) -> TokenStream{
	let kind = assert_tokenstream_itemkind!(kind,ItemKind::Const(..),&ItemKind::r#const());

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_first_ident = util::try_tokenstream!(enum_first_variant_ident(&item,"impl_enum_first"));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #const_ident: Self = #ident::#variant_first_ident;
		}
	}
}

#[cfg(feature = "attr_ends")]
pub fn gen_attr_last(
	ItemPrefix(attrs,vis,Concat((ref kind,const_ident))): ItemPrefix<Concat<(ItemKind,syn::Ident)>>,
	item: syn::ItemEnum
) -> TokenStream{
	let kind = assert_tokenstream_itemkind!(kind,ItemKind::Const(..),&ItemKind::r#const());

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let variant_last_ident = util::try_tokenstream!(enum_last_variant_ident(&item,"impl_enum_last"));

	quote!{
		#item

		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #const_ident: Self = #ident::#variant_last_ident;
		}
	}
}
