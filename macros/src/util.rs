use proc_macro2::Span;
use alloc::string::String;
use syn::{Fields,Ident,Variant};

#[cfg(any(feature = "derive_field_structs",feature = "attr_field_structs"))] pub mod occurs;
pub mod ident_attr_vis;

pub fn minimum_type_from_value(value: usize) -> Ident{
	if value <= u8::MAX as usize{
		Ident::new("u8",Span::call_site())
	}else if value <= u16::MAX as usize{
		Ident::new("u16",Span::call_site())
	}else if value <= u32::MAX as usize{
		Ident::new("u32",Span::call_site())
	}else if value <= u64::MAX as usize{
		Ident::new("u64",Span::call_site())
	}else{
		Ident::new("usize",Span::call_site())
	}
}

pub fn variant_unit_ident<'v>(variant: &'v Variant,derive_name: &'static str) -> &'v Ident{
	match variant.fields{
		Fields::Unit => &variant.ident,
		_ => panic!("`derive({})` may only be applied to enum items with only unit variants",derive_name)
	}
}

/// Simple conversion from a CamelCase string to a snake_case string.
///
/// It tries to follow the Rust naming conventions: <https://doc.rust-lang.org/1.0.0/style/style/naming/README.html>.
///
/// # Examples
/// ```rust,ignore
/// use enum_traits_macros::util::camelcase_to_snakecase;
/// assert_eq!(camelcase_to_snakecase("SnakeCaseStringIsItReadable") , "snake_case_string_is_it_readable");
/// assert_eq!(camelcase_to_snakecase("ANiceStringAndOKItIsIThink")  , "anice_string_and_okit_is_ithink");
/// assert_eq!(camelcase_to_snakecase("OptionalBTreeLeaf")           , "optional_btree_leaf");
/// assert_eq!(camelcase_to_snakecase("ALL_CAPS_AND_NOTHING_MORE")   , "all_caps_and_nothing_more");
/// assert_eq!(camelcase_to_snakecase("Some kind_of mix Of ALL_hEr") , "some kind_of mix of all_h_er");
/// ```
pub fn camelcase_to_snakecase<'s>(s: &'s str) -> String{
	let mut out = String::with_capacity(s.len() * 2);
	let mut cs = s.chars();
	let mut prev_is_separation = true;
	if let Some(c) = cs.next(){
		out.extend(c.to_lowercase());
		for c in cs{
			if c.is_uppercase(){
				if !prev_is_separation{out.push('_');}
				out.extend(c.to_lowercase());
				prev_is_separation = true;
			}else{
				out.push(c);
				prev_is_separation = c.is_ascii_punctuation() || c.is_ascii_whitespace();
			}
		}
	}
	out
}

