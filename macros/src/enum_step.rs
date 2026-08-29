use crate::util;
use proc_macro2::TokenStream;

fn gen_prev_match_arms<'i>(variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> impl Iterator<Item = TokenStream>{
	variants.clone()
		.map(|v| &v.ident)
		.zip(variants.map(|v| &v.ident).skip(1))
		.map(|(variant_ident1,variant_ident2)|{
			quote! { Self::#variant_ident2 => Self::#variant_ident1, }
		})
}

fn gen_next_match_arms<'i>(variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> impl Iterator<Item = TokenStream>{
	variants.clone()
		.map(|v| &v.ident)
		.zip(variants.map(|v| &v.ident).skip(1))
		.map(|(variant_ident1,variant_ident2)|{
			quote! { Self::#variant_ident1 => Self::#variant_ident2, }
		})
}

fn gen_prev_fn_body<'i>(variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> TokenStream{
	let match_arms = gen_prev_match_arms(variants);
	quote!{
		::core::option::Option::Some(match self{
			#( #match_arms )*
			_ => return ::core::option::Option::None
		})
	}
}

fn gen_next_fn_body<'i>(variants: impl Iterator<Item = &'i syn::Variant> + Clone) -> TokenStream{
	let match_arms = gen_next_match_arms(variants);
	quote!{
		::core::option::Option::Some(match self{
			#( #match_arms )*
			_ => return ::core::option::Option::None
		})
	}
}

#[cfg(feature = "derive_step")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumStep)"));

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	let prev_body = gen_prev_fn_body(item.variants.iter());
	let next_body = gen_next_fn_body(item.variants.iter());
	quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::EnumStep for #ident #ty_generics #where_clause{
			#[inline]
			#[allow(unreachable_code)]
			fn next(self) -> ::core::option::Option<Self>{
				#next_body
			}

			#[inline]
			#[allow(unreachable_code)]
			fn previous(self) -> ::core::option::Option<Self>{
				#prev_body
			}
		}
	}
}

#[cfg(feature = "attr_step")] use crate::util::parse::{DelimitedOpt,ItemPrefix};

#[cfg(feature = "attr_step")]
pub fn gen_attr_prev(
	DelimitedOpt((ItemPrefix(attrs,vis,mut sign),),default): DelimitedOpt<(ItemPrefix<syn::Signature>,),Option<syn::Expr>>,
	item: syn::ItemEnum
) -> TokenStream{
	use crate::util::replace_ty::replace_infer_ret;
	use syn::spanned::Spanned;

	sign.output = replace_infer_ret(sign.output,&syn::parse_quote!{ Self });

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	if util::is_option_ret(&sign.output){
		if let Some(default) = default{
			return syn::Error::new(default.span(),"`impl_enum_prev` with an `Option` as return type does not accept a parameter for the default value.").into_compile_error();
		}

		let body = gen_prev_fn_body(item.variants.iter());
		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{#body}
			}
		}
	}else{
		let Some(default) = default else{
			return syn::Error::new(sign.span(),"`impl_enum_prev` requires a parameter for the default value.").into_compile_error();
		};

		let match_arms = gen_prev_match_arms(item.variants.iter());
		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{
					match self{
						#( #match_arms )*
						_ => #default
					}
				}
			}
		}
	}
}

#[cfg(feature = "attr_step")]
pub fn gen_attr_next(
	DelimitedOpt((ItemPrefix(attrs,vis,mut sign),),default): DelimitedOpt<(ItemPrefix<syn::Signature>,),Option<syn::Expr>>,
	item: syn::ItemEnum
) -> TokenStream{
	use crate::util::replace_ty::replace_infer_ret;
	use syn::spanned::Spanned;

	sign.output = replace_infer_ret(sign.output,&syn::parse_quote!{ Self });

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;


	if util::is_option_ret(&sign.output){
		if let Some(default) = default{
			return syn::Error::new(default.span(),"`impl_enum_next` with an `Option` as return type does not accept a parameter for the default value.").into_compile_error();
		}

		let body = gen_next_fn_body(item.variants.iter());
		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{#body}
			}
		}
	}else{
		let Some(default) = default else{
			return syn::Error::new(sign.span(),"`impl_enum_next` requires a parameter for the default value.").into_compile_error();
		};

		let match_arms = gen_next_match_arms(item.variants.iter());
		quote!{
			#item

			#[automatically_derived]
			impl #impl_generics #ident #ty_generics #where_clause{
				#( #attrs )* #vis #sign{
					match self{
						#( #match_arms )*
						_ => #default
					}
				}
			}
		}
	}
}
