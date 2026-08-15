use crate::util;
use proc_macro2::{Literal,TokenStream};
use syn::{Ident,Lit,Variant};

fn variant_unit_ident(variant: &Variant) -> &Ident{
	util::variant_unit_ident(variant,"EnumIter")
}

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{//TODO: Consider rewriting output (EnumIter may not need Option, but then empty enums are not represented. Are they necessary to include?)
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;
	let visibility = &item.vis;

	let len = item.variants.len();
	let last = item.variants.last();

	/*let prev_match_arms = {
			let iter = item.variants.iter().rev().map(variant_unit_ident);
			iter.zip(item.variants.iter().rev().map(variant_unit_ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
		});*/

	let next_match_arms = {
			let iter = item.variants.iter().map(variant_unit_ident);
			iter.zip(item.variants.iter().map(variant_unit_ident).skip(1))
		}.map(|(variant_ident1,variant_ident2)|{
			quote! { &Some(#ident::#variant_ident1) => {self.0 = Some(#ident::#variant_ident2); #ident::#variant_ident2}, }
		});

	let len_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
		let i = Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = variant_unit_ident(variant);
		quote! { &#ident::#variant_ident => #i, }
	});

	let count_match_arms = item.variants.iter().enumerate().map(|(i,variant)|{
		let i = Lit::new(Literal::usize_unsuffixed(i));
		let variant_ident = variant_unit_ident(variant);
		quote! { #ident::#variant_ident => #i, }
	});

	let variant_first_ident = &item.variants.first().expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;
	//let variant_last_ident  = &item.variants.LAST.expect("`derive(EnumIter)` may only be applied to non-empty enums").ident;

	let struct_ident = format_ident!("{}Iter",ident);

	let struct_iter = quote!{
		#visibility struct #struct_ident #ty_generics #where_clause (pub Option<#ident #ty_generics>);
	};

	let impl_default = quote!{
		#[automatically_derived]
		impl #impl_generics ::core::default::Default for #struct_ident #ty_generics #where_clause{
			#[inline(always)]
			fn default() -> Self{#struct_ident (None)}
		}
	};

	let impl_iter = {
		let fn_next = quote!{
			#[inline]
			fn next(&mut self) -> Option<Self::Item>{
				Some(match &self.0{
					&None => {self.0 = Some(#ident::#variant_first_ident); #ident::#variant_first_ident},
					#( #next_match_arms )*
					_ => return None
				})
			}
		};

		let fn_size_hint = quote!{
			#[inline(always)]
			fn size_hint(&self) -> (usize,Option<usize>){
				use ::core::iter::ExactSizeIterator;
				(self.len(),Some(self.len()))
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

		let fn_last = if let Some(last_ident) = last.map(variant_unit_ident){quote!{
			#[inline(always)]
			fn last(self) -> Option<Self::Item>{
				Some(#ident::#last_ident)
			}
		}}else{quote!{
			#[inline(always)]
			fn last(self) -> Option<Self::Item>{
				None
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

	//TODO: May be an incorrect use of DoubleEndedIterator. Use Step instead
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

	let impl_intoiter = quote!{
		#[automatically_derived]
		impl #impl_generics ::enum_traits::Iterable for #ident #ty_generics #where_clause{
			type Iter = #struct_ident;
			#[inline(always)]fn variants() -> Self::Iter{#struct_ident(None)}
		}
	};

	quote!{
		#struct_iter
		#impl_intoiter
		#impl_default
		#impl_iter
		//#impl_diter
		#impl_exactiter
	}
}
