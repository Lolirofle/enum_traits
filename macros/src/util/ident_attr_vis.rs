use syn::parse::{Parse,ParseStream,Result};
use syn::parse::discouraged::Speculative;
use alloc::vec::Vec;

/// An identifier preceded by optional attributes and an optional visibility.
/// Examples:
/// - `#[cfg(feature = "x")] pub Name`
/// - `#[derive(Debug)] Name2`
/// - `pub(crate) Name3`
/// - `Name4`
pub struct IdentAttrVis{
	pub attrs: Vec<syn::Attribute>,
	pub vis: Option<syn::Visibility>,
	pub ident: syn::Ident,
}

impl Parse for IdentAttrVis{
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

		let ident = input.parse::<syn::Ident>()?;

		Ok(IdentAttrVis{attrs,vis,ident})
	}
}
