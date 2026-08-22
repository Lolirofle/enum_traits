use core::default::Default;
use core::fmt;
use proc_macro2::Span;
use alloc::string::String;
use syn::spanned::Spanned;

#[cfg(any(feature = "derive_field_structs",feature = "attr_field_structs"))] pub mod occurs;
pub mod parse;

pub fn minimum_type_from_value(value: usize) -> syn::Ident{
	if value <= u8::MAX as usize{
		syn::Ident::new("u8",Span::call_site())
	}else if value <= u16::MAX as usize{
		syn::Ident::new("u16",Span::call_site())
	}else if value <= u32::MAX as usize{
		syn::Ident::new("u32",Span::call_site())
	}else if value <= u64::MAX as usize{
		syn::Ident::new("u64",Span::call_site())
	}else{
		syn::Ident::new("usize",Span::call_site())
	}
}

pub fn check_unit_variant<'v,D>(variant: &'v syn::Variant,error_msg: D) -> syn::Result<()> where
	D: fmt::Display
{
	match variant.fields{
		syn::Fields::Unit => Ok(()),
		_ => Err(syn::Error::new(variant.span(),error_msg))
	}
}

pub fn check_unit_variants<'v,'s>(variants: impl Iterator<Item = &'v syn::Variant>,error_ctx: &'s str) -> syn::Result<()>{
	variants.map(|v| check_unit_variant(v,fmt::from_fn(|f| write!(f,"`{}` may only be applied to an enum item with only unit variants",error_ctx)))).collect()
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

pub fn filter_attributes<'a,I: 'a>(ident: I,it: impl IntoIterator<Item = &'a syn::Attribute>) -> impl Iterator<Item = &'a syn::Attribute> where
	syn::Ident: PartialEq<I>
{
	it.into_iter().filter(move |attr| attr.path().is_ident(&ident))
}

/*
use syn::parse::Parse;
pub fn parse_attributes<'a,I: 'a,T: Parse>(ident: I,it: impl IntoIterator<Item = &'a syn::Attribute>) -> impl Iterator<Item = syn::Result<T>> where
	Ident: PartialEq<I>
{
	it.into_iter().filter_map(move |attr|{
		if attr.path().is_ident(&ident){
			Some(attr.parse_args())
		}else{
			None
		}
	})
}
*/

pub fn parse_itemprefix_attributes<'a,const N: usize,I: 'a>(ident: I,item_kinds: [parse::ItemKind; N],it: impl IntoIterator<Item = &'a syn::Attribute>) -> syn::Result<[parse::ItemPrefix<Option<syn::Ident>>; N]> where
	syn::Ident: PartialEq<I>,
	[parse::ItemPrefix<Option<syn::Ident>>; N]: Default //TODO: Why is this not implemented in the general case in stdlib?
{
	let mut out: [_; _] = Default::default();
	'attr: for attr in filter_attributes(ident,it){
		let parse::ItemPrefix(a,v,parse::Successive((k,i))): parse::ItemPrefix<parse::Successive<(parse::ItemKind,syn::Ident)>> = attr.parse_args()?;
		for n in 0..N{
			if item_kinds[n] == k{
				out[n] = parse::ItemPrefix(a,v,Some(i));
				continue 'attr;
			}
		}
		return Err(syn::Error::new(attr.span(),alloc::format!("Unexpected item kind `{}` in attribute argument. Expected one of the following: {:?}",k,item_kinds)));
	}
	Ok(out)
}

macro_rules! try_tokenstream{
	($expr:expr $(,)?) => {
		match $expr{
			core::result::Result::Ok(x) => x,
			core::result::Result::Err(e) => return e.into_compile_error()
		}
	};
}
pub(crate) use try_tokenstream;
