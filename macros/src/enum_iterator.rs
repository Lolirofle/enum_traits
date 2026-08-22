use crate::util;
use proc_macro2::{Literal,TokenStream};

#[cfg(feature = "derive_iterator")]
pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let len = item.variants.len();
	let last = item.variants.last();

	util::try_tokenstream!(util::check_unit_variants(item.variants.iter(),"derive(EnumIterator)"));

	/*let prev_match_arms = {
			let iter = item.variants.iter().rev().map(|v| v.ident);
			iter.zip(item.variants.iter().rev().map(|v| v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
		});*/

	let next_match_arms = {
			let iter = item.variants.iter().map(|v| &v.ident);
			iter.zip(item.variants.iter().map(|v| &v.ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &mut #ident::#variant_ident1 => {*self = #ident::#variant_ident2; #ident::#variant_ident2}, }
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

	let impl_iter = {
		let fn_next = quote!{
			#[inline]
			#[allow(unreachable_code)]
			fn next(&mut self) -> ::core::option::Option<Self::Item>{
				::core::option::Option::Some(match self{
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
			#[inline]
			fn count(self) -> usize{
				#len - 1 - match self{
					#( #count_match_arms )*
				}
			}
		};

		let fn_last = match last.map(|v| &v.ident){
			::core::option::Option::Some(last_ident) => quote!{
				#[inline(always)]
				fn last(self) -> ::core::option::Option<Self::Item>{
					::core::option::Option::Some(#ident::#last_ident)
				}
			},
			::core::option::Option::None => quote!{
				#[inline(always)]
				fn last(self) -> ::core::option::Option<Self::Item>{
					::core::option::Option::None
				}
			},
		};

		quote!{
			#[automatically_derived]
			impl #impl_generics ::core::iter::Iterator for #ident #ty_generics #where_clause{
				type Item = Self;
				#fn_next
				#fn_count
				#fn_size_hint
				#fn_last
			}
		}
	};

	/*let impl_diter = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::iter::DoubleEndedIterator for #ident #ty_generics #where_clause{
			#[inline]
			#[allow(unreachable_code)]
			fn next_back(&mut self) -> Option<Self::Item>{
				Some(match self{
					#( #prev_match_arms )*
					_ => return None
				})
			}
		}
	};*/

	let impl_eiter = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::iter::ExactSizeIterator for #ident #ty_generics #where_clause{
			#[inline]
			fn len(&self) -> usize{
				#len - 1 - match self{
					#( #len_match_arms )*
				}
			}
		}
	};

	quote!{
		#impl_iter
		//#impl_diter
		#impl_eiter
	}
}
