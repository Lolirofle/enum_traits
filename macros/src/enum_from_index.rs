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
	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumFromIndex)"));

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(item.variants.iter());
	let match_arms2 = gen_match_arms(item.variants.iter());

	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::EnumFromIndex for #ident #ty_generics #where_clause{
			#[inline]
			fn from_index(index: <Self as ::enum_traits::EnumIndex>::Type) -> ::core::option::Option<Self>{
				::core::option::Option::Some(match index{
					#( #match_arms )*
					_ => return ::core::option::Option::None
				})
			}

			#[inline]
			unsafe fn from_index_unchecked(index: <Self as ::enum_traits::EnumIndex>::Type) -> Self{
				match index{
					#( #match_arms2 )*
					_ => unreachable!()
				}
			}
		}
	}
}

#[cfg(feature = "attr_from_index")]
use crate::util::parse::{DelimitedOpt,ItemPrefix};

#[cfg(feature = "attr_from_index")]
pub fn gen_attr(
	DelimitedOpt((ItemPrefix(attrs,vis,mut sign),),default): DelimitedOpt<(ItemPrefix<syn::Signature>,),Option<syn::Expr>>,
	item: syn::ItemEnum
) -> TokenStream{
	use crate::util::replace_ty::replace_infer_ret;
	use syn::spanned::Spanned;

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"impl_enum_from_index_default"));
	if sign.inputs.len() == 1 && let Some(syn::FnArg::Typed(param)) = sign.inputs.first_mut(){
		*param.pat = syn::parse_quote!{ index }; //Replaces pattern to a binding of our choice.
	}else{
		return syn::Error::new(sign.inputs.span(),"`impl_enum_from_index` expects a function signature with a single parameter of a numeric type.").into_compile_error();
	}
	sign.output = replace_infer_ret(sign.output,&syn::parse_quote!{ Self });

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let match_arms = gen_match_arms(item.variants.iter());

	if util::is_option_ret(&sign.output){
		if let Some(default) = default{
			return syn::Error::new(default.span(),"`impl_enum_from_index` with an `Option` as return type does not accept a parameter for the default value.").into_compile_error();
		}

		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{
					::core::option::Option::Some(match index{
						#( #match_arms )*
						_ => return ::core::option::Option::None
					})
				}
			}
		}
	}else{
		let Some(default) = default else{
			return syn::Error::new(sign.span(),"`impl_enum_from_index` requires a parameter for the default value.").into_compile_error();
		};

		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{
					match index{
						#( #match_arms )*
						_ => #default
					}
				}
			}
		}
	}
}
