use proc_macro2::TokenStream;
use syn::{Fields,FieldsNamed,FieldsUnnamed,Ident,Index,Member,Variant};

fn fields_to_ty_pat_expr(item_ident: &Ident,variant: &Variant) -> (TokenStream,TokenStream,TokenStream){(
	//Type
	match variant.fields{
		Fields::Unit => quote! { () },
		Fields::Unnamed(FieldsUnnamed{unnamed: ref fields,..}) |
		Fields::Named(FieldsNamed{named: ref fields,..}) => {
			match fields.len(){
				1 => {
					let ty = &fields.first().unwrap().ty;
					quote! { #ty }
				},
				_ => {
					let tys = fields.iter().map(|field| &field.ty);
					quote! { (#(#tys),*) }
				},
			}
		}
	},

	//Pattern
	match variant.fields{
		Fields::Unit => quote! { () },
		Fields::Unnamed(FieldsUnnamed{unnamed: ref fields,..}) |
		Fields::Named(FieldsNamed{named: ref fields,..}) => {
			match fields.len(){
				1 => quote! { x0 },
				_ => {
					let vars = fields.iter()
						.enumerate()
						.map(|(i,_field)| format_ident!("x{}",i));
					quote! { ( #(#vars),* ) }
				},
			}
		}
	},

	//Expression
	{
		let variant_ident = &variant.ident;
		match variant.fields{
			Fields::Unit => quote! { #item_ident::#variant_ident },
			Fields::Unnamed(FieldsUnnamed{unnamed: ref fields,..}) => {
				let args = fields.iter()
					.enumerate()
					.map(|(i,_field)| format_ident!("x{}",i));
				quote! { #item_ident::#variant_ident( #(#args),* ) }
			},
			Fields::Named(FieldsNamed{named: ref fields,..}) => {
				let args = fields.iter()
					.enumerate()
					.map(|(i,_field)| format_ident!("x{}",i));
				let field_names = fields.iter()
					.enumerate()
					.map(|(i,field)| match field.ident{
						Some(ref ident) => Member::Named(ident.clone()),
						None => Member::Unnamed(Index::from(i)),
					});
				quote! { #item_ident::#variant_ident{ #(#field_names : #args),* } }
			},
		}
	},
)}

pub fn gen_impl(item: syn::ItemEnum) -> TokenStream{
	let (impl_generics,ty_generics,where_clause) = item.generics.split_for_impl();
	let ident = item.ident;

	let impls = item.variants.iter().filter_map(|variant|{
		let (ty,pattern,expr) = fields_to_ty_pat_expr(&ident,&variant);
		Some(quote!{
			#[automatically_derived]
			impl #impl_generics ::core::convert::From<#ty> for #ident #ty_generics #where_clause{
				#[inline(always)]
				fn from(#pattern: #ty) -> Self{
					#expr
				}
			}
		})
	});

	quote!{#( #impls )*}
}
