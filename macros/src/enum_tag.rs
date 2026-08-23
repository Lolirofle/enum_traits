use crate::util;
use crate::util::parse::{Concat,ItemKind,ItemPrefix,assert_itemkind};
use proc_macro2::TokenStream;
use syn::ext::IdentExt as _;

//TODO: Attributes and naming on the generated variants
//TODO: Maybe split the attirbute arguments into multiple different attributes instead?

fn gen_unit_variants<'v>(variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	variants.map(|variant|{
		let variant_ident = &variant.ident;
		quote! { #variant_ident, }
	})
}

fn gen_match_arms<'v>(unit_enum_ident: &syn::Ident,variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	use syn::Fields;
	variants.map(move |variant|{
		let variant_ident = &variant.ident;
		match variant.fields{
			Fields::Unit       => quote! { Self::#variant_ident     => #unit_enum_ident::#variant_ident, },
			Fields::Unnamed(_) => quote! { Self::#variant_ident(..) => #unit_enum_ident::#variant_ident, },
			Fields::Named(_)   => quote! { Self::#variant_ident{..} => #unit_enum_ident::#variant_ident, },
		}
	})
}

fn gen_match_arms_is<'v>(unit_enum_ident: &syn::Ident,variants: impl Iterator<Item = &'v syn::Variant>) -> impl Iterator<Item = TokenStream>{
	use syn::Fields;
	variants.map(move |variant|{
		let variant_ident = &variant.ident;
		match variant.fields{
			Fields::Unit       => quote! { #unit_enum_ident::#variant_ident => if let Self::#variant_ident     = self{true}else{false}, },
			Fields::Unnamed(_) => quote! { #unit_enum_ident::#variant_ident => if let Self::#variant_ident(..) = self{true}else{false}, },
			Fields::Named(_)   => quote! { #unit_enum_ident::#variant_ident => if let Self::#variant_ident{..} = self{true}else{false}, },
		}
	})
}

fn gen_unit_enum(ItemPrefix(unit_enum_attrs,unit_enum_vis,Concat(kind,unit_enum_ident)): ItemPrefix<Concat<ItemKind,syn::Ident>>,item: &syn::ItemEnum) -> TokenStream{
	let unit_variants = gen_unit_variants(item.variants.iter());
	quote!{
		#[automatically_derived]
		#(#unit_enum_attrs)*
		#unit_enum_vis #kind #unit_enum_ident{
			#( #unit_variants )*
		}
	}
}

fn find_tag_attribute_name(item: &syn::ItemEnum) -> syn::Result<Option<syn::Ident>>{
	Ok(match util::find_unique_attribute("enum_tag",item.attrs.iter())?{
		Some(attr) => {
			let mut out = None;
			attr.parse_nested_meta(|meta|{
				if meta.path.is_ident("name"){
					let prefix; syn::parenthesized!(prefix in meta.input);
					let prefix: ItemPrefix<syn::Ident> = prefix.parse()?;
					out = Some(prefix.2);
				}
				Ok(())
			})?;
			out
		},
		None => None
	})
}

#[cfg(feature = "derive_tag")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;

	//Extract unit enum identifier from attribute if possible.
	//Generate unit enum otherwise.
	let (unit_enum_ident,unit_enum) = match util::try_tokenstream!(find_tag_attribute_name(&item)){
		Some(ident) => (ident,TokenStream::default()),
		None => {
			let unit_enum_ident = format_ident!("{}Tag",ident.unraw());
			(
				unit_enum_ident.clone(),
				gen_unit_enum(
					ItemPrefix(
						alloc::vec![syn::parse_quote!(#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)])],
						item.vis.clone(),
						Concat(
							ItemKind::Enum(syn::parse_quote!(enum)),
							unit_enum_ident
						)
					),
					&item
				),
			)
		},
	};

	//Generation

	let match_arms = gen_match_arms(&unit_enum_ident,item.variants.iter());
	let match_arms_into = gen_match_arms(&unit_enum_ident,item.variants.iter());

	quote!{
		#unit_enum

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

#[cfg(feature = "attr_tag")]
pub fn gen_attr(attr: TokenStream,item: syn::ItemEnum) -> TokenStream{
	use syn::parse::Parser;
	use syn::spanned::Spanned;

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let span = attr.span();

	let mut out = TokenStream::default();
	let mut unit_enum_ident: Option<syn::Ident> = None;
	util::try_tokenstream!(syn::meta::parser(|meta|{
		if meta.path.is_ident("name"){
			//Parse args
			let prefix; syn::parenthesized!(prefix in meta.input);
			let mut prefix: ItemPrefix<Concat<ItemKind,syn::Ident>> = prefix.parse()?;
			prefix.2.0 = assert_itemkind!(prefix.2.0,ItemKind::Enum(..),ItemKind::r#enum());

			unit_enum_ident = Some(prefix.2.1.clone());
			out.extend(gen_unit_enum(prefix,&item))
		}else if meta.path.is_ident("to"){
			if let Some(ref unit_enum_ident) = unit_enum_ident{
				//Parse args
				let prefix; syn::parenthesized!(prefix in meta.input);
				let ItemPrefix(attrs,vis,Concat(kind,fn_ident)): ItemPrefix<Concat<ItemKind,syn::Ident>> = prefix.parse()?;
				let kind = assert_itemkind!(kind,ItemKind::Fn(..),ItemKind::const_fn());

				let match_arms = gen_match_arms(&unit_enum_ident,item.variants.iter());

				out.extend(quote!{
					#[automatically_derived]
					impl #impl_generics #ident #ty_generics #where_clause{
						#( #attrs )* #vis #kind #fn_ident(&self) -> #unit_enum_ident{
							match self{
								#( #match_arms )*
							}
						}
					}
				})
			}else{
				return Err(syn::Error::new(span,"`name` field in attribute `enum_tag` must be defined before `to`."))
			}
		}else if meta.path.is_ident("is"){
			if let Some(ref unit_enum_ident) = unit_enum_ident{
				//Parse args
				let prefix; syn::parenthesized!(prefix in meta.input);
				let ItemPrefix(attrs,vis,Concat(kind,fn_ident)): ItemPrefix<Concat<ItemKind,syn::Ident>> = prefix.parse()?;
				let kind = assert_itemkind!(kind,ItemKind::Fn(..),ItemKind::const_fn());

				let match_arms = gen_match_arms_is(&unit_enum_ident,item.variants.iter());

				out.extend(quote!{
					#[automatically_derived]
					impl #impl_generics #ident #ty_generics #where_clause{
						#( #attrs )* #vis #kind #fn_ident<const __EnumTag_param: #unit_enum_ident>(&self) -> bool{
							match __EnumTag_param{
								#( #match_arms )*
							}
						}
					}
				})
			}else{
				return Err(syn::Error::new(span,"`name` field in attribute `enum_tag` must be defined before `is`."))
			}
		}else{
			return Err(syn::Error::new(span,"Unknown field in attribute `enum_tag`."))
		}
		Ok(())
	}).parse2(attr));
	quote!{
		#item
		#out
	}
}
