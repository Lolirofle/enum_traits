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

macro_rules! try_tokenstream{
	($expr:expr $(,)?) => {
		match $expr{
			core::result::Result::Ok(x) => x,
			core::result::Result::Err(e) => return e.into_compile_error()
		}
	};
}
pub(crate) use try_tokenstream;

/*
pub struct TokensResult<T>(pub syn::Result<T>);
impl<T: quote::ToTokens> quote::ToTokens for TokensResult<T>{
	fn to_tokens(&self,tokens: &mut proc_macro2::TokenStream){
		match self.0{
			Ok(ref x) => x.to_tokens(tokens),
			Err(ref e) => e.to_compile_error().to_tokens(tokens),
		}
	}
}
*/

macro_rules! attr_params{
	($name:ident , $attrs:expr , $($idents:ident),* $(,)? ; $($keys:ident : $tys:ty),* $(,)?) => {{
		struct AttrParams{
			$($idents: bool,)*
			$($keys: Option<$tys>,)*
		}
		let mut params = AttrParams{
			$($idents: false,)*
			$($keys: None,)*
		};
		crate::util::filter_attributes(stringify!($name),$attrs.iter()).map(|attr|
			attr.parse_nested_meta(|meta|{
				$(
					if meta.path.is_ident(stringify!($idents)){
						if params.$idents{
							return Err(meta.error(concat!("Duplicate parameter ",stringify!($idents)," in attribute ",stringify!($name))));
						}
						params.$idents = true;
						return Ok(());
					}
				)*
				$(
					if meta.path.is_ident(stringify!($keys)) {
						if let Some(_) = params.$keys{
							return Err(meta.error(concat!("Duplicate parameter ",stringify!($keys)," in attribute ",stringify!($name))));
						}
						let content;
						syn::parenthesized!(content in meta.input);
						params.$keys = Some(content.parse()?);
						return Ok(());
					}
				)*
				Err(meta.error(concat!("Unrecognised parameter in attribute ",stringify!($name))))
			})
		).collect::<syn::Result<()>>().map(|()| params)
	}};
}
pub(crate) use attr_params;
