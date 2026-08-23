use syn::parse::{Parse,ParseStream,Result};
use syn::parse::discouraged::Speculative;
use alloc::vec::Vec;

pub struct Successive<T>(pub T);
impl Parse for Successive<()>{
	fn parse(_: ParseStream) -> Result<Self>{
		Ok(Successive(()))
	}
}
impl<A: Parse,B: Parse> Parse for Successive<(A,B)>{
	fn parse(input: ParseStream) -> Result<Self>{
		let a = input.parse::<A>()?;
		let b = input.parse::<B>()?;
		Ok(Successive((a,b)))
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
#[derive(Default)]
pub struct ItemPrefix<I>(pub Vec<syn::Attribute>,pub Option<syn::Visibility>,pub I);

impl<I: Parse> Parse for ItemPrefix<I>{
	fn parse(input: ParseStream) -> Result<Self>{
		//Attributes
		let mut attrs = Vec::new();
		while input.peek(syn::Token![#]){
			attrs.extend(input.call(syn::Attribute::parse_outer)?);
		}

		//Visibility
		let fork = input.fork();
		let vis = if let Ok(v) = fork.parse::<syn::Visibility>(){
			input.advance_to(&fork);
			Some(v)
		}else{
			None
		};

		let ident = input.parse::<I>()?;

		Ok(ItemPrefix(attrs,vis,ident))
	}
}

//#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum ItemKind{
	None,
	Enum(syn::Token![enum]),
	Const(syn::Token![const]),
	Fn(Option<syn::Token![const]>,Option<syn::Token![async]>,syn::Safety,Option<syn::Abi>,syn::Token![fn]),
	Struct(syn::Token![struct]),
	Impl(syn::Token![impl]),
}

impl ItemKind{
	pub fn default_fn() -> Self{
		ItemKind::Fn(None,None,syn::Safety::Default,None,core::default::Default::default())
	}

	pub fn or(self,default: Self) -> Self{match self{
		ItemKind::None => default,
		k => k
	}}
}

impl core::fmt::Display for ItemKind{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result{use ItemKind::*; match self{
		None       => write!(f,"none"),
		Enum  (..) => write!(f,"enum"),
		Const (..) => write!(f,"const"),
		Fn    (..) => write!(f,"fn"),
		Struct(..) => write!(f,"struct"),
		Impl  (..) => write!(f,"impl"),
	}}
}

impl Parse for ItemKind{
	fn parse(input: ParseStream) -> Result<Self>{
		fn parse_fn_after_const(input: ParseStream) -> Result<(Option<syn::Token![async]>,syn::Safety,Option<syn::Abi>,Option<syn::Token![fn]>)>{
			match (input.parse()?,syn::Safety::parse_safe_or_unsafe(input)?,input.parse()?,input.parse()?){
				t@(_,_,_,Some(_)) | t@(None,syn::Safety::Default,None,None) => Ok(t),
				_ => Err(syn::Error::new(input.span(),"expected fn after fn related keywords"))

			}
		}

		{
			use ItemKind::*;
			Ok(if input.peek(syn::Token![enum]){
				Enum(input.parse::<syn::Token![enum]>()?)
			}else if input.peek(syn::Token![const]){
				let a = input.parse::<syn::Token![const]>()?;
				if let (b,c,d,Option::Some(e)) = parse_fn_after_const(input)?{
					Fn(Option::Some(a),b,c,d,e)
				}else{
					Const(a)
				}
			}else if let (b,c,d,Option::Some(e)) = parse_fn_after_const(input)?{
				Fn(Option::None,b,c,d,e)
			}else if input.peek(syn::Token![struct]){
				Struct(input.parse::<syn::Token![struct]>()?)
			}else if input.peek(syn::Token![impl]){
				Impl(input.parse::<syn::Token![impl]>()?)
			}else{
				None
			})
		}
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
