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

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum ItemKind{
	None,
	Enum,
	Const,
	Fn,
	Struct,
	Impl,
	ImplFor,
}

impl core::fmt::Display for ItemKind{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result{use ItemKind::*; match self{
		None    => write!(f,"none"),
		Enum    => write!(f,"enum"),
		Const   => write!(f,"const"),
		Fn      => write!(f,"fn"),
		Struct  => write!(f,"struct"),
		Impl    => write!(f,"impl"),
		ImplFor => write!(f,"impl for"),
	}}
}

impl Parse for ItemKind{
	fn parse(input: ParseStream) -> Result<Self>{
		use ItemKind::*;
		Ok(if input.peek(syn::Token![enum]){
			input.parse::<syn::Token![enum]>()?;
			Enum
		}else if input.peek(syn::Token![const]){
			input.parse::<syn::Token![const]>()?;
			Const
		}else if input.peek(syn::Token![fn]){
			input.parse::<syn::Token![fn]>()?;
			Fn
		}else if input.peek(syn::Token![struct]){
			input.parse::<syn::Token![struct]>()?;
			Struct
		}else if input.peek(syn::Token![impl]){
			input.parse::<syn::Token![impl]>()?;
			if input.peek(syn::Token![for]){
				input.parse::<syn::Token![for]>()?;
				ImplFor
			}else{
				Impl
			}
		}else{
			None
		})
	}
}
