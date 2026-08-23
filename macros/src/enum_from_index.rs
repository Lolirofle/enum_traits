use crate::util;
use proc_macro2::{Literal,TokenStream};

fn gen_match_arms<'v>(variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	variants.enumerate().map(|(i,variant)| {
		let i = syn::Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = &variant.ident;
		quote! { #i => Self::#variant_ident, }
	})
}

#[cfg(feature = "derive_from_index")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumFromIndex)"));

	let match_arms = gen_match_arms(item.variants.iter());
	let match_arms2 = gen_match_arms(item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::FromIndex for #ident #ty_generics #where_clause{
			#[inline]
			fn from_index(index: <Self as ::enum_traits::Index>::Type) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match index{
					#( #match_arms )*
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

#[cfg(feature = "attr_from_index")]
pub fn gen_attr_default(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util::parse::{Concat,Delimited,ItemKind,ItemPrefix};
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"impl_enum_from_index_default"));
	let Delimited((ItemPrefix(attrs,vis,Concat(kind,fn_ident)),index_ty,default)) = util::try_tokenstream!(syn::parse2::<Delimited<(ItemPrefix<Concat<ItemKind,syn::Ident>>,syn::Type,syn::Expr)>>(attr));
	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #fn_ident(index: #index_ty) -> Self{
				match index{
					#( #match_arms )*
					_ => #default
				}
			}
		}
	}
}

#[cfg(feature = "attr_from_index")]
pub fn gen_attr_optional(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use crate::util::parse::{Concat,Delimited,ItemKind,ItemPrefix};
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"impl_enum_from_index"));
	let Delimited((ItemPrefix(attrs,vis,Concat(kind,fn_ident)),index_ty)) = util::try_tokenstream!(syn::parse2::<Delimited<(ItemPrefix<Concat<ItemKind,syn::Ident>>,syn::Type)>>(attr));
	let kind = kind.or(ItemKind::r#fn());
	let match_arms = gen_match_arms(item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics #ident #ty_generics #where_clause{
			#( #attrs )* #vis #kind #fn_ident(index: #index_ty) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match index{
					#( #match_arms )*
					_ => return ::core::option::Option::None
				})
			}
		}
	}
}
