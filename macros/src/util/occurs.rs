use syn::visit::Visit;

pub struct OccursVisit<I>{
	pub ident: I,
	pub found: bool,
}

impl<I> OccursVisit<I>{
	#[inline]
	pub fn new(ident: I) -> Self{OccursVisit{
		ident,
		found: false,
	}}
}

impl<'ast> Visit<'ast> for OccursVisit<&'ast syn::Ident>{
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

impl<'ast> Visit<'ast> for OccursVisit<&'ast syn::Lifetime>{
	fn visit_lifetime(&mut self,lifetime: &'ast syn::Lifetime){
		if self.ident == lifetime{
			self.found = true;
		}
	}

	fn visit_trait_bound(&mut self,tb: &'ast syn::TraitBound){
		if let Some(it) = &tb.lifetimes{
			let mut o = OccursVisit::new(self.ident);
			o.visit_bound_lifetimes(it);
			if o.found{
				return
			}
		}
		self.visit_path(&tb.path);
	}
}

pub struct TypeOccursVisit<'ast>(pub OccursVisit<&'ast syn::Ident>);

impl<'ast> TypeOccursVisit<'ast>{
	#[inline]
	pub fn new(ident: &'ast syn::Ident) -> Self{TypeOccursVisit(OccursVisit{
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
