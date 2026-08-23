use alloc::vec::Vec;
use core::default::Default;
use syn::parse::{Parse,ParseStream,Result};

pub struct Concat<A,B>(pub A,pub B);
impl<A: Parse,B: Parse> Parse for Concat<A,B>{
	fn parse(input: ParseStream) -> Result<Self>{
		let a = input.parse::<A>()?;
		let b = input.parse::<B>()?;
		Ok(Concat(a,b))
	}
}

pub struct Delimited<T>(pub T);
macro_rules! impl_parse_delimited{
	($head:ident $(,$tail:ident)*) => {
		impl<$head: Parse $(,$tail: Parse)*> Parse for Delimited<($head,$($tail,)*)>{
			fn parse(input: ParseStream) -> Result<Self>{
				let out = (
					input.parse::<$head>()?,
					$(
						{
							let _: syn::Token![,] = input.parse()?;
							input.parse::<$tail>()?
						},
					)*
				);
				//Optional trailing comma.
				if input.peek(syn::Token![,]){
					let _: syn::Token![,] = input.parse()?;
				}
				Ok(Delimited(out))
			}
		}
	};
}
impl_parse_delimited!(A,B);
impl_parse_delimited!(A,B,C);
impl_parse_delimited!(A,B,C,D);
impl_parse_delimited!(A,B,C,D,E);
impl_parse_delimited!(A,B,C,D,E,F);
impl_parse_delimited!(A,B,C,D,E,F,G);
impl_parse_delimited!(A,B,C,D,E,F,G,H);

/// An identifier preceded by optional attributes and an optional visibility.
/// Should loosely follow the initial parts of `syn::ItemConst` and `syn::TraitItemConst`.
/// Examples:
/// - `#[cfg(feature = "x")] pub Name`
/// - `#[derive(Debug)] Name2`
/// - `pub(crate) Name3`
/// - `Name4`
//#[derive(Default)]
pub struct ItemPrefix<I>(pub Vec<syn::Attribute>,pub syn::Visibility,pub I);

impl<I: Parse> Parse for ItemPrefix<I>{
	fn parse(input: ParseStream) -> Result<Self>{
		//Attributes
		let mut attrs = Vec::new();
		while input.peek(syn::Token![#]){
			attrs.extend(input.call(syn::Attribute::parse_outer)?);
		}
		let vis = input.parse::<syn::Visibility>()?;
		let ident = input.parse::<I>()?;

		Ok(ItemPrefix(attrs,vis,ident))
	}
}

#[derive(Clone)]
pub enum ItemKind{
	None,
	Enum(syn::Token![enum]),
	Const(syn::Token![const]),
	Fn(Option<syn::Token![const]>,Option<syn::Token![async]>,syn::Safety,Option<syn::Abi>,syn::Token![fn]),
	Struct(syn::Token![struct]),
	Impl(syn::Token![impl]),
}

impl ItemKind{
	pub fn or(self,default: Self) -> Self{match self{
		ItemKind::None => default,
		k => k
	}}

	pub fn span(&self) -> proc_macro2::Span{use ItemKind::*; match self{
		None         => unimplemented!(),
		Enum  (..,t) => t.span,
		Const (..,t) => t.span,
		Fn    (..,t) => t.span,
		Struct(..,t) => t.span,
		Impl  (..,t) => t.span,
	}}

	#[allow(unused)] pub fn r#enum()   -> Self{ItemKind::Enum(Default::default())}
	#[allow(unused)] pub fn r#const()  -> Self{ItemKind::Const(Default::default())}
	#[allow(unused)] pub fn r#fn()     -> Self{ItemKind::Fn(None,None,syn::Safety::Default,None,Default::default())}
	#[allow(unused)] pub fn const_fn() -> Self{ItemKind::Fn(Some(Default::default()),None,syn::Safety::Default,None,Default::default())}
	#[allow(unused)] pub fn r#struct() -> Self{ItemKind::Struct(Default::default())}
	#[allow(unused)] pub fn r#impl()   -> Self{ItemKind::Impl(Default::default())}
}

impl Parse for ItemKind{
	fn parse(input: ParseStream) -> Result<Self>{
		use ItemKind::*;
		Ok(if input.peek(syn::Token![enum]){
			Enum(input.parse::<syn::Token![enum]>()?)
		}else if input.peek(syn::Token![struct]){
			Struct(input.parse::<syn::Token![struct]>()?)
		}else if input.peek(syn::Token![impl]){
			Impl(input.parse::<syn::Token![impl]>()?)
		}else{match (input.parse::<Option<syn::Token![const]>>()?,input.parse::<Option<syn::Token![async]>>()?,syn::Safety::parse_safe_or_unsafe(input)?,input.parse::<Option<syn::Abi>>()?,input.parse::<Option<syn::Token![fn]>>()?){
			(Option::Some(a),Option::None,syn::Safety::Default,Option::None,Option::None) => Const(a),
			(Option::None   ,Option::None,syn::Safety::Default,Option::None,Option::None) => None,
			(a,b,c,d,Option::Some(e)) => Fn(a,b,c,d,e),
			_ => return Err(syn::Error::new(input.span(),"expected fn after function qualifiers")),
		}})
	}
}

impl quote::ToTokens for ItemKind{
	fn to_tokens(&self,tokens: &mut proc_macro2::TokenStream){use ItemKind::*; match self{
		None => (),
		Enum(t) => t.to_tokens(tokens),
		Const(t) => t.to_tokens(tokens),
		Fn(a,b,c,d,e) => {
			a.to_tokens(tokens);
			b.to_tokens(tokens);
			c.to_tokens(tokens);
			d.to_tokens(tokens);
			e.to_tokens(tokens);
		},
		Struct(t) => t.to_tokens(tokens),
		Impl(t) => t.to_tokens(tokens),
	}}
}

//Checks if the first arg (ItemKind) is matching the second arg pattern (ItemKind), becoming the first arg.
//Or become the third arg if the first arg is itemKind::None.
//For use in Result<_> functions.
macro_rules! assert_itemkind{
	($got:expr,$expected:pat,$default:expr) => {
		match $got{
			ItemKind::None => $default,
			$expected      => $got,
			_              => return Err(syn::Error::new($got.span(),"Unexpected item kind"))
		}
	};
}
pub(crate) use assert_itemkind;

//Checks if the first arg (ItemKind) is matching the second arg pattern (ItemKind), becoming the first arg.
//Or become the third arg if the first arg is itemKind::None.
//For use in TokenStream functions.
macro_rules! assert_tokenstream_itemkind{
	($got:expr,$expected:pat,$default:expr) => {
		match $got{
			ItemKind::None => $default,
			$expected      => $got,
			_              => return syn::Error::new($got.span(),"Unexpected item kind").into_compile_error()
		}
	};
}
pub(crate) use assert_tokenstream_itemkind;
