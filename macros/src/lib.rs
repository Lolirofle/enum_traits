#![crate_type="dylib"]
#![feature(plugin,plugin_registrar,rustc_private)]
#![plugin(quasi_macros)]

extern crate aster;
extern crate syntax;
extern crate rustc;
extern crate quasi;

use aster::AstBuilder;
//use aster::ident::ToIdent;
use syntax::ast;
use syntax::attr;
use syntax::codemap::{Span,Spanned};
use syntax::ext::base::{Annotatable,ExtCtxt};
use syntax::ptr;
use rustc::plugin::Registry;
//use std::{iter,slice};

fn minimum_type_from_value(context: &ExtCtxt,value: usize) -> ptr::P<ast::Ty>{
	if value <= u8::max_value() as usize{
		quote_ty!(context,u8)
	}else if value <= u16::max_value() as usize{
		quote_ty!(context,u16)
	}else if value <= u32::max_value() as usize{
		quote_ty!(context,u32)
	}else if value <= u64::max_value() as usize{
		quote_ty!(context,u64)
	}else{
		quote_ty!(context,usize)
	}
}

fn type_from_repr_attr<'i,I>(context: &ExtCtxt,attrs: I) -> Option<ptr::P<ast::Ty>>
	where I: Iterator<Item = &'i Spanned<ast::Attribute_>>
{
	for attr in attrs{
		for repr in attr::find_repr_attrs(&context.parse_sess.span_diagnostic,attr).iter(){
			use syntax::ast::{IntTy,UintTy};
			use syntax::attr::{IntType,ReprAttr};

			return Some(match repr{
				&ReprAttr::ReprInt(_,IntType::SignedInt(IntTy::TyI8))     => quote_ty!(context,i8),
				&ReprAttr::ReprInt(_,IntType::SignedInt(IntTy::TyI16))    => quote_ty!(context,i16),
				&ReprAttr::ReprInt(_,IntType::SignedInt(IntTy::TyI32))    => quote_ty!(context,i32),
				&ReprAttr::ReprInt(_,IntType::SignedInt(IntTy::TyI64))    => quote_ty!(context,i64),
				&ReprAttr::ReprInt(_,IntType::UnsignedInt(UintTy::TyU8))  => quote_ty!(context,u8),
				&ReprAttr::ReprInt(_,IntType::UnsignedInt(UintTy::TyU16)) => quote_ty!(context,u16),
				&ReprAttr::ReprInt(_,IntType::UnsignedInt(UintTy::TyU32)) => quote_ty!(context,u32),
				&ReprAttr::ReprInt(_,IntType::UnsignedInt(UintTy::TyU64)) => quote_ty!(context,u64),
				_ => continue
			});
		}
	}

	None
}

#[allow(non_snake_case)]
fn expand_derive_EnumIndex(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{ref variants},ref generics) => {
				let builder = AstBuilder::new().span(span);

				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				//Borrow the where clause
				let where_clause = &generics.where_clause;

				//Output type by type representation from `repr(..)` or minimum type required to store
				let output_type = type_from_repr_attr(context,item.attrs.iter()).unwrap_or(minimum_type_from_value(context,variants.len()));

				//Push the generated `impl` item
				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::enum_traits::Index for $ty $where_clause{
						type Type = $output_type;
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumFromIndex)` may only be applied to enum items");
}

#[allow(non_snake_case)]
fn expand_derive_EnumFromIndex(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{ref variants},ref generics) => {
				let builder = AstBuilder::new().span(span);

				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				//Borrow the where clause
				let where_clause = &generics.where_clause;

				let mut error = false;
				let mut match_arms: Vec<_> = variants.into_iter().enumerate().map(|(i,variant)|{
					//Variant path
					let variant_path = builder.path()
						.segment(&item.ident).build()
						.segment(variant.node.name).build()
						.build();

					//Pattern
					match variant.node.data{
						ast::VariantData::Tuple(ref args,_) if args.is_empty() => (),
						_ => {
							//Wrong application
							context.span_err(meta_item.span,"`derive(EnumFromIndex)` may only be applied to enum items with no fields");
							error = true;
						}
					};

					let i = Spanned{node: ast::Lit_::LitInt(i as u64,ast::LitIntType::UnsuffixedIntLit(ast::Sign::Plus)),span: span};
					quote_arm!(context,
						$i => Some($variant_path),
					)
				}).collect();
				if error{return;}
				match_arms.push(quote_arm!(context,
					_ => None,
				));

				//Push the generated `impl` item
				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::enum_traits::FromIndex for $ty $where_clause{
						#[inline]
						fn from_index(index: <Self as ::enum_traits::Index>::Type) -> Option<Self>{
							match index{$match_arms}
						}
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumFromIndex)` may only be applied to enum items");
}

#[allow(non_snake_case)]
fn expand_derive_EnumToIndex(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{ref variants},ref generics) => {
				let builder = AstBuilder::new().span(span);

				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				//Borrow the where clause
				let where_clause = &generics.where_clause;

				let match_arms: Vec<_> = variants.into_iter().enumerate().map(|(i,variant)|{
					//Variant path
					let variant_path = builder.path()
						.segment(&item.ident).build()
						.segment(variant.node.name).build()
						.build();

					//Pattern
					let variant_pattern = match variant.node.data{
						ast::VariantData::Tuple(..) |
						ast::VariantData::Unit(..)  => quote_pat!(context,$variant_path(..)),
						ast::VariantData::Struct(..) => quote_pat!(context,$variant_path{..}),
					};

					quote_arm!(context,
						$variant_pattern => $i as <Self as ::enum_traits::Index>::Type,
					)
				}).collect();

				//Push the generated `impl` item
				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::enum_traits::ToIndex for $ty $where_clause{
						#[inline]
						fn into_index(self) -> <Self as ::enum_traits::Index>::Type{
							match self{$match_arms}
						}

						#[inline]
						fn index(&self) -> <Self as ::enum_traits::Index>::Type{
							match *self{$match_arms}
						}
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumToIndex)` may only be applied to enum items");
}

#[allow(non_snake_case)]
fn expand_derive_EnumLen(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{ref variants},ref generics) => {
				let builder = AstBuilder::new().span(span);

				let len = variants.len();

				//Borrow the where clause
				let where_clause = &generics.where_clause;

				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::enum_traits::Len for $ty $where_clause{
						const LEN: usize = $len;
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumLen)` may only be applied to enum items");
	return;
}

#[allow(non_snake_case)]
fn expand_derive_EnumIterator(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{..},ref generics) => {
				let builder = AstBuilder::new().span(span);

				//Borrow the where clause
				let where_clause = &generics.where_clause;
//TODO: Should only work for enums with no struct fields
				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::std::iter::Iterator for $ty $where_clause{
						type Item = Self;
						#[inline(always)]fn next(&mut self) -> Option<Self>{
							Self::from_index(self.index()+1)
						}
						#[inline(always)]fn size_hint(&self) -> (usize,Option<usize>){
							use ::std::iter::ExactSizeIterator;
							(self.len(),Some(self.len()))
						}
					}
				).unwrap()));
				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::std::iter::DoubleEndedIterator for $ty $where_clause{
						#[inline]fn next_back(&mut self) -> Option<Self::Item>{
							self.index().checked_sub(1).and_then(|i| Self::from_index(i))
						}
					}
				).unwrap()));
				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::std::iter::ExactSizeIterator for $ty $where_clause{
						#[inline(always)]fn len(&self) -> usize{<Self as enum_traits::Len>::LEN}
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumIterator)` may only be applied to enum items");
	return;
}

#[allow(non_snake_case)]
fn expand_derive_EnumEnds(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ast::EnumDef{ref variants},ref generics) => {
				let builder = AstBuilder::new().span(span);

				let first = match variants.first(){Some(v) => v,None => {context.span_err(meta_item.span,"`derive(EnumEnds)` may only be applied to non-empty enums");;return;}};
				let last  = variants.last().unwrap();

				//Variant paths
				let first_path = builder.path()
					.segment(&item.ident).build()
					.segment(first.node.name).build()
					.build();
				let last_path = builder.path()
					.segment(&item.ident).build()
					.segment(last.node.name).build()
					.build();

				//Borrow the where clause
				let where_clause = &generics.where_clause;
//TODO: Should only work for enums with no struct fields
				//Type of the enum
				let ty_path = builder.path()
					.segment(&item.ident).with_generics(generics.clone()).build()
					.build();
				let ty = builder.ty().build_path(ty_path.clone());

				push(Annotatable::Item(quote_item!(context,
					#[automatically_derived]
					impl $generics ::enum_traits::Ends for $ty $where_clause{
						#[inline(always)]fn first() -> Self{$first_path}
						#[inline(always)]fn last()  -> Self{$last_path}
					}
				).unwrap()));
				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`derive(EnumEnds)` may only be applied to enum items");
	return;
}

/*
fn enum_stripped_variants(enum_def: ast::EnumDef) -> ast::EnumDef{
	ast::EnumDef{
		variants: enum_def.variants.into_iter().map(|variant| variant.map(|spanned| Spanned{
			node: ast::Variant_{
				kind: ast::VariantData::Tuple(Vec::new()),
				..spanned.node
			},
			..spanned
		})).collect()
	}
}

fn structdef_field_visibility(struct_def: ast::StructDef,visibility: ast::Visibility) -> ast::StructDef{
	ast::StructDef{
		fields: struct_def.fields.iter().map(|field|{
			let mut field = field.clone();
			field.node.data = match field.node.data{
				ast::StructFieldKind::NamedField(ident,_) => ast::StructFieldKind::NamedField(ident,visibility),
				ast::StructFieldKind::UnnamedField(_)     => ast::StructFieldKind::UnnamedField(visibility)
			};
			field
		}).collect(),
		..struct_def
	}
}

fn enumvariants_to_structdefs<'l>(enum_def: &'l ast::EnumDef) -> iter::Map<slice::Iter<'l,ptr::P<Spanned<ast::Variant_>>>,fn(&ptr::P<ast::Variant>) -> ptr::P<ast::StructDef>>{
	use aster::struct_def::{StructDefBuilder,StructFieldBuilder};

	fn map_variant_to_structdef(variant: &ptr::P<ast::Variant>) -> ptr::P<ast::StructDef>{
		fn map_variantarg_to_structfield(variant_arg: &ast::VariantArg) -> Spanned<ast::StructField_>{
			StructFieldBuilder::unnamed().pub_().build_ty(
				variant_arg.ty.clone()
			)
		}

		match variant.node.data{
			ast::VariantData::Tuple(ref variant_args,_) => ptr::P(ast::StructDef{
				ctor_id: Some(ast::DUMMY_NODE_ID),
				..(*StructDefBuilder::new().with_fields(
					variant_args.into_iter().map(map_variantarg_to_structfield as fn(&_) -> _)
				).build()).clone()
			}),
			ast::VariantData::Struct(ref struct_def) => {
				struct_def.clone().map(|struct_def| structdef_field_visibility(struct_def,ast::Visibility::Public))
			},
		}
	}

	enum_def.variants.iter().map(map_variant_to_structdef as fn(&ptr::P<ast::Variant>) -> ptr::P<ast::StructDef>)
}

fn expand_enum_variant_structs(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: &Annotatable,push: &mut FnMut(Annotatable)){
	if let &Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ref enum_def,ref generics) => {
				for (struct_def,variant) in enumvariants_to_structdefs(enum_def).zip(enum_def.variants.iter()){
					push(Annotatable::Item(ptr::P(ast::Item{
						ident: variant.node.name,
						attrs: vec!(),
						id   : ast::DUMMY_NODE_ID,
						node : ast::Item_::ItemStruct(struct_def,generics.clone()),
						vis  : item.vis.clone(),
						span : span,
					})))
				}

				return;
			},
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`enum_variant_structs` may only be applied to enum items");
	return;
}

fn expand_enum_as_separate_mod(context: &mut ExtCtxt,span: Span,meta_item: &ast::MetaItem,annotatable: Annotatable) -> Annotatable{
	use syntax::ast_util::empty_generics;

	if let Annotatable::Item(ref item) = annotatable{
		match item.node{
			ast::Item_::ItemEnum(ref enum_def,_/*TODO: ref generics*/) => return Annotatable::Item(ptr::P(ast::Item{
				ident: item.ident,
				attrs: vec!(quote_attr!(context,#[allow(non_snake_case)])),
				id   : ast::DUMMY_NODE_ID,
				node : {
					ast::Item_::ItemMod(ast::Mod{
						inner: span,
						items: {
							let mut items = Vec::new();

							expand_enum_variant_structs(context,span,meta_item,&annotatable,&mut |annotatable| match annotatable{
								Annotatable::Item(item) => {
									items.push(item.map(|item| ast::Item{vis: ast::Visibility::Public,..item}));
								},
								_ => ()
							});
							items.push(ptr::P(ast::Item{
								ident: "Kind".to_ident(),
								attrs: vec!(),
								id   : ast::DUMMY_NODE_ID,
								node : ast::Item_::ItemEnum(enum_stripped_variants(enum_def.clone()),empty_generics()),
								vis  : ast::Visibility::Public,
								span : item.span.clone(),
							}));

							items
						},
					})
				},
				vis  : item.vis.clone(),
				span : item.span.clone(),
			})),
			_ => {}
		}
	}

	//Wrong application
	context.span_err(meta_item.span,"`separate_enum_as_mod` may only be applied to enum items");
	return annotatable;
}
*/

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry){
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumIndex"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumIndex))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumFromIndex"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumFromIndex))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumToIndex"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumToIndex))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumLen"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumLen))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumIterator"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumIterator))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_EnumEnds"),
		syntax::ext::base::MultiDecorator(Box::new(expand_derive_EnumEnds))
	);
	/*reg.register_syntax_extension(
		syntax::parse::token::intern("enum_variant_structs"),
		syntax::ext::base::MultiDecorator(Box::new(expand_enum_variant_structs))
	);
	reg.register_syntax_extension(
		syntax::parse::token::intern("enum_as_separate_mod"),
		syntax::ext::base::MultiModifier(Box::new(expand_enum_as_separate_mod))
	);*/
}
