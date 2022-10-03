use alloc::vec;
use alloc::vec::Vec;
use core::default::Default;
use core::hash::Hash;
use hashbrown::HashSet;
use syn::{Ident,Lifetime};
use syn::visit as vis;

pub struct FreeVars<Var>{
	vars: HashSet<Var>,
	ignore_scopes: Vec<HashSet<Var>>,
}

impl<Var> FreeVars<Var> where
	Var: Copy + Eq + Hash
{
	#[inline]
	pub fn new() -> Self{FreeVars{
		vars: HashSet::new(),
		ignore_scopes: {
			let mut vec = Vec::with_capacity(8);
			vec.push(HashSet::new());
			vec
		},
	}}

	#[inline]
	fn scope(&mut self){
		self.ignore_scopes.push(HashSet::new());
	}

	#[inline]
	fn unscope(&mut self){
		self.ignore_scopes.pop();
	}

	#[inline]
	fn latest_ignore_scope(&mut self) -> Option<&mut HashSet<Var>>{
		self.ignore_scopes.last_mut()
	}

	#[inline]
	pub fn ignore(&mut self,var: Var){
		if let Some(ignore_scope) = self.latest_ignore_scope(){
			ignore_scope.insert(var);
		}
	}

	pub fn is_ignored(&self,var: Var) -> bool{
		self.ignore_scopes.iter().any(|ignore_scope| ignore_scope.contains(&var))
	}

	#[inline]
	pub fn register(&mut self,var: Var){
		if !self.is_ignored(var){
			self.vars.insert(var);
		}
	}

	#[inline]
	pub fn is_free(&self,var: Var) -> bool{
		self.vars.contains(&var)
	}

	#[inline]
	pub fn free_iter<'s>(&'s self) -> impl Iterator<Item = Var> + 's{
		self.vars.iter().copied()
	}
}

pub struct FreeVarsVisit<'ast>{
	pub idents: FreeVars<&'ast Ident>,
	pub lifetimes: FreeVars<&'ast Lifetime>,
}

impl<'ast> FreeVarsVisit<'ast>{
	#[inline]
	pub fn new() -> Self{FreeVarsVisit{
		idents: FreeVars::new(),
		lifetimes: FreeVars::new(),
	}}

	#[inline]
	fn scope(&mut self){
		self.idents.scope();
		self.lifetimes.scope();
	}

	#[inline]
	fn unscope(&mut self){
		self.idents.unscope();
		self.lifetimes.unscope();
	}
}

impl<'ast> vis::Visit<'ast> for FreeVarsVisit<'ast>{
	///////////////////////////////////////////////////////////////////////////
	// Base

	fn visit_ident(&mut self,ident: &'ast Ident){
		self.idents.register(ident);
	}

	fn visit_lifetime(&mut self,lifetime: &'ast Lifetime){
		self.lifetimes.register(lifetime);
	}

	fn visit_path(&mut self,path: &'ast syn::Path){match path.get_ident(){
		//When a path is an ident, it can be a free variable.
		Some(ident) => self.visit_ident(ident),

		//When a path is not an ident, it may include arguments.
		None => vis::visit_path(self,path)
	}}

	fn visit_path_segment(&mut self,path_segment: &'ast syn::PathSegment){
		//Ignore the ident in a path segment because it cannot be a free variable (visit_path already takes care of the single path segment case).
		self.visit_path_arguments(&path_segment.arguments);
	}

	///////////////////////////////////////////////////////////////////////////
	// Scopes
	// (Nodes that capture a new environment)

	//Because it may contain `BoundLifetimes` (from `PredicateType`, `WherePredicate`).
	fn visit_where_predicate(&mut self,node: &'ast syn::WherePredicate){
		self.scope();
		vis::visit_where_predicate(self,node);
		self.unscope();
	}

	//Because it may contain `BoundLifetimes`.
	fn visit_type(&mut self,node: &'ast syn::Type){
		self.scope();
		vis::visit_type(self,node);
		self.unscope();
	}

	//Because it may contain `BoundLifetimes`.
	fn visit_trait_bound(&mut self,node: &'ast syn::TraitBound){
		self.scope();
		vis::visit_trait_bound(self,node);
		self.unscope();
	}

	//Because it may contain `PatIdent`.
	fn visit_arm(&mut self,node: &'ast syn::Arm){
		self.scope();
		vis::visit_arm(self,node);
		self.unscope();
	}


	//Because it may contain `PatIdent`.
	fn visit_expr_closure(&mut self,node: &'ast syn::ExprClosure){
		self.scope();
		vis::visit_expr_closure(self,node);
		self.unscope();
	}

	//Because it may contain `PatIdent` (from `Pat`, `Stmt`, `Local`, `Block`).
	fn visit_block(&mut self,node: &'ast syn::Block){
		self.scope();
		vis::visit_block(self,node);
		self.unscope();
	}

	///////////////////////////////////////////////////////////////////////////
	// Var registering
	// (Nodes that bound new variables)

	fn visit_bound_lifetimes(&mut self,node: &'ast syn::BoundLifetimes){
		for def in node.lifetimes.iter(){
			self.lifetimes.ignore(&def.lifetime);
		}

		vis::visit_bound_lifetimes(self,node)
	}

	fn visit_pat_ident(&mut self,node: &'ast syn::PatIdent){
		self.idents.ignore(&node.ident);
		vis::visit_pat_ident(self,node)
	}

	fn visit_label(&mut self,node: &'ast syn::Label){
		self.lifetimes.ignore(&node.name);
		vis::visit_label(self,node)
	}

	fn visit_type_param(&mut self,node: &'ast syn::TypeParam){
		self.idents.ignore(&node.ident);
		for ref bound in node.bounds{
			self.visit_type_param_bound(bound);
		}
		if let Some(ref def) = node.default{
			self.visit_type(def);
		}
	}

	fn visit_lifetime_def(&mut self,node: &'ast syn::LifetimeDef){
		self.lifetimes.ignore(&node.lifetime);
		for ref bound in node.bounds{
			self.visit_lifetime(bound);
		}
	}

	fn visit_const_param(&mut self,node: &'ast syn::ConstParam){
		self.idents.ignore(&node.ident);
		self.visit_type(&node.ty);
		if let Some(ref def) = node.default{
			self.visit_expr(def);
		}
	}

	//TODO: Local can refer to earlier definitions of the same name (let x = x + 1. The inner x in this stmt is a different x), while function definitions cannot (fn f(){f()} refers to the same f)
	//TODO: ItemConst, ItemEnum, ItemFn, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemUnion, TraitItem*, ForeignItem*

	///////////////////////////////////////////////////////////////////////////
	// Ignore

	fn visit_attribute(&mut self,_: &'ast syn::Attribute){}
	fn visit_binding(&mut self,node: &'ast syn::Binding){
		//Ignore visiting Binding.ident because it does not refer to a defined variable.
		self.visit_type(&node.ty)
	}
	fn visit_constraint(&mut self,node: &'ast syn::Constraint){
		//Ignore visiting Constraint.ident because it does not refer to a defined variable.
		for bound in node.bounds.iter(){
			self.visit_type_param_bound(bound);
		}
	}
	fn visit_field(&mut self,node: &'ast syn::Field){
		//Ignore visiting Field.ident because it is not possible to refer to it using a single ident path.
		self.visit_type(&node.ty)
	}
	fn visit_member(&mut self,_: &'ast syn::Member){}
	fn visit_use_tree(&mut self,_: &'ast syn::UseTree){}
	fn visit_variant(&mut self,node: &'ast syn::Variant){
		//Ignore visiting Variant.ident because it is not possible to refer to it using a single ident path.
		self.visit_fields(&node.fields);
		if let Some(it) = &node.discriminant{
			self.visit_expr(&(it).1);
		}
	}
}

impl<'ast> FreeVarsVisit<'ast>{
	pub fn filter_generics(&self,generics: &mut syn::Generics){
		generics.params = generics.params.iter().filter_map(|param| self.filter_generic_param(param)).collect();
		if let Some(ref mut w) = generics.where_clause{
			w.predicates = w.predicates.iter().filter_map(|pred| self.filter_where_predicate(pred)).collect();
		}
	}

	fn filter_generic_param(&self,param: &syn::GenericParam) -> Option<syn::GenericParam>{match param{
		syn::GenericParam::Type(param) => self.filter_type_param(param).map(syn::GenericParam::Type),
		syn::GenericParam::Lifetime(param) => self.filter_lifetime_def(param).map(syn::GenericParam::Lifetime),
		syn::GenericParam::Const(param) => self.filter_const_param(param).map(syn::GenericParam::Const),
	}}

	fn filter_type_param(&self,param: &syn::TypeParam) -> Option<syn::TypeParam>{
		
	}

	fn filter_lifetime_def(&self,param: &syn::LifetimeDef) -> Option<syn::LifetimeDef>{
		if self.lifetimes.is_free(&param.lifetime){
			let mut param = param.clone();
			param.bounds = param.bounds.iter().filter(|bound| self.lifetimes.is_free(bound)).cloned().collect();
			Some(param)
		}else{
			None
		}
	}

	fn filter_const_param(&self,param: &syn::ConstParam) -> Option<syn::ConstParam>{
		if self.idents.is_free(&param.ident){
			let mut param = param.clone();
			param.bounds = param.bounds.iter().filter(|bound| self.lifetimes.is_free(bound)).cloned().collect();
			Some(param)
		}else{
			None
		}
	}

	fn filter_where_predicate(&self,param: &syn::WherePredicate) -> Option<syn::WherePredicate>{
		
	}
}

/*
#[cfg(test)]
mod tests{
	#[test]
	fn
}
*/
