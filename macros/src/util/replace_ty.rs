use core::convert::Into as _;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

pub fn replace_infer_ty(mut ty: syn::Type,replace: &syn::Type) -> syn::Type{
	struct ReplaceVisit<'t>(&'t syn::Type);
	impl<'ast> VisitMut for ReplaceVisit<'ast>{
		fn visit_type_mut(&mut self,ty: &mut syn::Type){
			if let syn::Type::Infer(_) = ty{
				*ty = self.0.clone();
			}else{
				syn::visit_mut::visit_type_mut(self,ty);
			}
		}
	}

	ReplaceVisit(replace).visit_type_mut(&mut ty);
	ty
}

pub fn replace_infer_ret(ty: syn::ReturnType,replace: &syn::Type) -> syn::ReturnType{
	match ty{
		syn::ReturnType::Default => syn::ReturnType::Type(syn::Token![->](ty.span()),replace.clone().into()),
		syn::ReturnType::Type(arrow,mut ty) => {
			*ty = replace_infer_ty(*ty,replace);
			syn::ReturnType::Type(arrow,ty)
		},
	}
}
