use proc_macro2::Span;
use alloc::string::String;
use syn::{Fields,Ident,Variant};

pub mod free_vars;
//pub mod occurs;

pub fn minimum_type_from_value(value: usize) -> Ident{
	if value <= u8::max_value() as usize{
		Ident::new("u8",Span::call_site())
	}else if value <= u16::max_value() as usize{
		Ident::new("u16",Span::call_site())
	}else if value <= u32::max_value() as usize{
		Ident::new("u32",Span::call_site())
	}else if value <= u64::max_value() as usize{
		Ident::new("u64",Span::call_site())
	}else{
		Ident::new("usize",Span::call_site())
	}
}

/*
/**
 * Extracts the type from `repr(u*)` or `repr(i*)` attributes if it exists.
 */
pub fn type_from_repr_attr<'i,I>(attrs: I) -> Option<Ident>
	where I: Iterator<Item = &'i Attribute>
{
	use syn::{Meta,MetaList,NestedMeta};

	for attr in attrs{match attr.parse_meta(){
		Ok(Meta::List(MetaList{path,nested,..})) if path.is_ident("repr") => {
			for meta in nested{
				if let NestedMeta::Meta(Meta::Path(repr)) = meta{
					if let Some(repr) = repr.get_ident(){
						let repr = repr.to_string();
						if repr.as_str() == "usize" || repr.as_str() == "isize" || {
							let mut repr_chars = repr.chars();
							repr_chars.next().map_or(false , |c| c == 'u' || c == 'i') //Starts with an 'u' or 'i'.
							&& repr_chars.next().map_or(false , |c| c.is_digit(10)) //Exists a digit after.
							&& repr_chars.all(|c| c.is_digit(10)) //All after are digits too.
						}{
							return Some(Ident::new(repr.as_ref(),Span::call_site()));
						}
					}
				}
				continue;
			}
		},
		_ => {continue;},
	}}
	None
}
*/

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

/*
pub fn ident_to_path(ident: Ident) -> syn::Path{syn::Path{
	leading_colon: None,
	segments: {
		let mut p = syn::punctuated::Punctuated::new();
		p.push(syn::PathSegment{
			ident,
			arguments: syn::PathArguments::None
		});
		p},
}}

#[inline]
pub fn idents_to_path<Idents: Iterator<Item = Ident>>(leading_colon: bool,idents: Idents) -> syn::Path{
	path_segments_to_path(
		leading_colon,
		idents.map(|ident| syn::PathSegment{
			ident: ident,
			arguments: syn::PathArguments::None
		})
	)
}

#[inline]
pub fn path_segments_to_path<PathSegments: Iterator<Item = syn::PathSegment>>(leading_colon: bool,path_segments: PathSegments) -> syn::Path{syn::Path{
	leading_colon: if leading_colon {Some(Default::default())} else {None},
	segments: path_segments.collect(),
}}

pub fn ident_to_expr(ident: Ident) -> Expr{ExprPath{
	attrs: Vec::new(),
	qself: None,
	path: ident_to_path(ident),
}.into()}

pub fn minimum_type_containing_enum(item: &syn::ItemEnum) -> syn::Ident{//TODO: Maybe useful to export?
	//First, check if there's a repr attribute
	type_from_repr_attr(item.attrs.iter())
	.unwrap_or_else(||
		//Second, use the maximum value of an explicit discriminant or the length of the enum (depending on which is the greatest)
		minimum_type_from_value(match item.variants.iter().filter_map(|variant| match variant.discriminant{
				Some((_,syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(ref discrimimant) , ..}))) => Some(discrimimant.base10_parse::<usize>().expect("Discriminant cannot be made into an usize")),
				_ => None
			}).max(){
				Some(max) => cmp::max(cmp::max(item.variants.len(),1)-1 , max),
				//Third, use the length of the enum
				_ => cmp::max(item.variants.len(),1)-1
			}
		)
	)
}

#[inline]
pub fn generics_add_type_param(generics: &mut syn::Generics,param: syn::TypeParam){
	let pos = generics.params.iter().position(|param| if let syn::GenericParam::Type(_) = param {true} else {false});
	let param = param.into();
	if let Some(pos) = pos{
		generics.params.insert(pos,param);
	}else{
		generics.params.push(param);
	}
}
*/
