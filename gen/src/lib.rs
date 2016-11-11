#![feature(plugin,plugin_registrar,rustc_private)]
#![crate_type = "dylib"]

extern crate proc_macro_plugin;
extern crate rustc_plugin;
extern crate syntax;

use syntax::ext::proc_macro_shim::prelude::*;

use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry){
	reg.register_syntax_extension(token::intern("EnumUnitVariants"),SyntaxExtension::AttrProcMacro(Box::new(gen_EnumUnitVariants)));
}

fn gen_EnumUnitVariants(attr: TokenStream,item: TokenStream) -> TokenStream{item}

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
			ast::ItemKind::Enum(ref enum_def,ref generics) => {
				for (struct_def,variant) in enumvariants_to_structdefs(enum_def).zip(enum_def.variants.iter()){
					push(Annotatable::Item(ptr::P(ast::Item{
						ident: variant.node.name,
						attrs: vec!(),
						id   : ast::DUMMY_NODE_ID,
						node : ast::ItemKind::ItemStruct(struct_def,generics.clone()),
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
			ast::ItemKind::Enum(ref enum_def,_/*TODO: ref generics*/) => return Annotatable::Item(ptr::P(ast::Item{
				ident: item.ident,
				attrs: vec!(quote_attr!(context,#[allow(non_snake_case)])),
				id   : ast::DUMMY_NODE_ID,
				node : {
					ast::ItemKind::ItemMod(ast::Mod{
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
								node : ast::ItemKind::Enum(enum_stripped_variants(enum_def.clone()),empty_generics()),
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
