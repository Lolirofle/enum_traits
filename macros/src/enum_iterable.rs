use crate::util;
use proc_macro2::{Literal,TokenStream};
use syn::ext::IdentExt as _;
use syn::spanned::Spanned as _;

#[cfg(feature = "derive_iterable")]
pub fn gen_derive(item: syn::ItemEnum) -> TokenStream{//TODO: Consider rewriting output (EnumIterable may not need Option, but then empty enums are not represented. Are they necessary to include?)
	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumIterable)"));

	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = &item.ident;
	let visibility = &item.vis;

	let len = item.variants.len();
	let last = item.variants.last();

	/*let prev_match_arms = {
			let iter = item.variants.iter().rev().map(|v| v.ident);
			iter.zip(item.variants.iter().rev().map(|v| v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
		});*/

	let next_match_arms = {
			let iter = item.variants.iter().map(|v| &v.ident);
			iter.zip(item.variants.iter().map(|v| &v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &::core::option::Option::Some(#ident::#variant_ident1) => {self.0 = ::core::option::Option::Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
		});

	let len_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
		let i = syn::Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = &variant.ident;
		quote! { &#ident::#variant_ident => #i, }
	});

	let count_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
		let i = syn::Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = &variant.ident;
		quote! { #ident::#variant_ident => #i, }
	});

	let variant_first_ident = &util::try_tokenstream!(item.variants.first().ok_or_else(|| syn::Error::new(item.span(),"`derive(EnumIterable)` may only be applied to non-empty enums"))).ident;

	let struct_ident = format_ident!("{}Iter",ident.unraw());

	let struct_iter = quote!{
		#visibility struct #struct_ident #ty_generics #where_clause (pub ::core::option::Option<#ident #ty_generics>);
	};

	let impl_default = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::default::Default for #struct_ident #ty_generics #where_clause{
			#[inline(always)]
			fn default() -> Self{#struct_ident (::core::option::Option::None)}
		}
	};

	let impl_iter = {
		let fn_next = quote!{
			#[inline]
			fn next(&mut self) -> ::core::option::Option<Self::Item>{
				::core::option::Option::Some(match &self.0{
					&::core::option::Option::None => {self.0 = ::core::option::Option::Some(#ident::#variant_first_ident); #ident::#variant_first_ident},
					#( #next_match_arms )*
					_ => return ::core::option::Option::None
				})
			}
		};

		let fn_size_hint = quote!{
			#[inline(always)]
			fn size_hint(&self) -> (usize,::core::option::Option<usize>){
				use ::core::iter::ExactSizeIterator;
				(self.len(),::core::option::Option::Some(self.len()))
			}
		};

		let fn_count = quote!{
			#[inline(always)]
			fn count(self) -> usize{
				self.0.map_or(#len,|variant|{
					#len - 1 - match variant{
						#( #count_match_arms )*
					}
				})
			}
		};

		let fn_last = if let ::core::option::Option::Some(last_ident) = last.map(|v| &v.ident){quote!{
			#[inline(always)]
			fn last(self) -> ::core::option::Option<Self::Item>{
				::core::option::Option::Some(#ident::#last_ident)
			}
		}}else{quote!{
			#[inline(always)]
			fn last(self) -> ::core::option::Option<Self::Item>{
				::core::option::Option::None
			}
		}};

		quote!{
			#[automatically_derived]
			impl #impl_generics ::core::iter::Iterator for #struct_ident #ty_generics #where_clause{
				type Item = #ident;

				#fn_next
				#fn_size_hint
				#fn_count
				#fn_last
			}
		}
	};

	//TODO: May be an incorrect use of DoubleEndedIterator. Use Step instead?
	/*let impl_diter = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::iter::Iterator for ::core::iter::Rev<#struct_ident> #ty_generics #where_clause{
			#[inline]
			fn next_back(&mut self) -> Option<Self::Item>{
				Some(match &self.0{
					&None => {self.0 = Some(#ident::#variant_last_ident); #ident::#variant_last_ident},
					#( #prev_match_arms )*
					_ => return None
				})
			}
		}
	};*/

	let impl_exactiter = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::iter::ExactSizeIterator for #struct_ident #ty_generics #where_clause{
			#[inline]
			fn len(&self) -> usize{
				self.0.as_ref().map_or(#len,|variant|{
					#len - 1 - match variant{
						#( #len_match_arms )*
					}
				})
			}
		}
	};

	let impl_iterable = quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Iterable for #ident #ty_generics #where_clause{
			type Iter = #struct_ident #ty_generics;
			#[inline(always)]fn variants() -> Self::Iter{#struct_ident(::core::option::Option::None)}
		}
	};

	quote!{
		#struct_iter
		#impl_iterable
		#impl_default
		#impl_iter
		//#impl_diter
		#impl_exactiter
	}
}
