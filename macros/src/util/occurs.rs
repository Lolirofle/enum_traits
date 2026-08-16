use syn::visit::Visit;

pub struct IdentOccursVisit<'ast>{
	pub ident: &'ast syn::Ident,
	pub found: bool,
}

impl<'ast> IdentOccursVisit<'ast>{
	#[inline]
	pub fn new(ident: &'ast syn::Ident) -> Self{IdentOccursVisit{
		ident,
		found: false,
	}}
}

impl<'ast> Visit<'ast> for IdentOccursVisit<'ast>{
	fn visit_ident(&mut self,ident: &'ast syn::Ident){
		if self.ident == ident{
			self.found = true;
		}
	}

	fn visit_path(&mut self,p: &'ast syn::Path){
		if let Some(ps) = p.segments.last(){
			self.visit_path_segment(ps);
		}
	}
}

pub struct TypeOccursVisit<'ast>(pub IdentOccursVisit<'ast>);

impl<'ast> TypeOccursVisit<'ast>{
	#[inline]
	pub fn new(ident: &'ast syn::Ident) -> Self{TypeOccursVisit(IdentOccursVisit{
		ident,
		found: false,
	})}
}

impl<'ast> Visit<'ast> for TypeOccursVisit<'ast>{
	fn visit_type(&mut self,ty: &'ast syn::Type){
		self.0.visit_type(ty);
	}

	fn visit_trait_bound(&mut self,tb: &'ast syn::TraitBound){
		if let Some(it) = &tb.lifetimes{
			let mut o = TypeOccursVisit::new(self.0.ident);
			o.visit_bound_lifetimes(it);
			if o.0.found{
				return
			}
		}
		self.visit_path(&tb.path);
	}
}

pub struct LifetimeOccursVisit<'ast>{
	pub lifetime: &'ast syn::Lifetime,
	pub found: bool,
}

impl<'ast> LifetimeOccursVisit<'ast>{
	#[inline]
	pub fn new(lifetime: &'ast syn::Lifetime) -> Self{LifetimeOccursVisit{
		lifetime,
		found: false,
	}}
}

impl<'ast> Visit<'ast> for LifetimeOccursVisit<'ast>{
	fn visit_lifetime(&mut self,lifetime: &'ast syn::Lifetime){
		if self.lifetime == lifetime{
			self.found = true;
		}
	}

	fn visit_trait_bound(&mut self,tb: &'ast syn::TraitBound){
		if let Some(it) = &tb.lifetimes{
			let mut o = LifetimeOccursVisit::new(self.lifetime);
			o.visit_bound_lifetimes(it);
			if o.found{
				return
			}
		}
		self.visit_path(&tb.path);
	}
}
