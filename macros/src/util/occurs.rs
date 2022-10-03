use alloc::boxed::Box;
use syn::punctuated::Punctuated;

pub trait Occurs<Tree>{
	fn occurs(&self,tree: &Tree) -> bool;
}

// Generic implementations

impl<Tree,Name> Occurs<Box<Tree>> for Name where
	Name: Occurs<Tree>
{
	#[inline] fn occurs(&self,tree: &Box<Tree>) -> bool{
		self.occurs(tree.as_ref())
	}
}

impl<Tree,Name> Occurs<Option<Tree>> for Name where
	Name: Occurs<Tree>
{
	#[inline] fn occurs(&self,tree: &Option<Tree>) -> bool{
		tree.as_ref().map_or(false,|tree| self.occurs(tree))
	}
}

impl<Tree,Delim,Name> Occurs<Punctuated<Tree,Delim>> for Name where
	Name: Occurs<Tree>
{
	#[inline] fn occurs(&self,tree: &Punctuated<Tree,Delim>) -> bool{
		tree.iter().any(|tree| self.occurs(tree))
	}
}



//Specific syn type implementations
//(Base cases)

impl Occurs<syn::Ident> for syn::Ident{
	#[inline] fn occurs(&self,other: &syn::Ident) -> bool{
		self == other
	}
}

impl Occurs<syn::Lifetime> for syn::Ident{
	#[inline] fn occurs(&self,_: &syn::Lifetime) -> bool{
		false
	}
}

impl Occurs<syn::Path> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::Path) -> bool{match tree.get_ident(){
		Some(ident) => self.occurs(ident),
		None => self.occurs(&tree.segments)
	}}
}

impl Occurs<syn::Ident> for syn::Lifetime{
	#[inline] fn occurs(&self,_: &syn::Ident) -> bool{
		false
	}
}

impl Occurs<syn::Lifetime> for syn::Lifetime{
	#[inline] fn occurs(&self,other: &syn::Lifetime) -> bool{
		self == other
	}
}

//Specific syn type implementations
//(Inductive cases)

impl<Name> Occurs<syn::Field> for Name where
	Name: Occurs<syn::Type>
{
	#[inline] fn occurs(&self,tree: &syn::Field) -> bool{
		self.occurs(&tree.ty)
	}
}

impl<Name> Occurs<syn::Type> for Name where
	Name: Occurs<syn::TypeArray>,
	Name: Occurs<syn::TypeBareFn>,
	Name: Occurs<syn::TypeGroup>,
	Name: Occurs<syn::TypeImplTrait>,
	Name: Occurs<syn::TypeParen>,
	Name: Occurs<syn::TypePath>,
	Name: Occurs<syn::TypePtr>,
	Name: Occurs<syn::TypeReference>,
	Name: Occurs<syn::TypeSlice>,
	Name: Occurs<syn::TypeTraitObject>,
	Name: Occurs<syn::TypeTuple>,
{
	#[inline] fn occurs(&self,tree: &syn::Type) -> bool{use syn::Type::*; match tree{
		Array(ty) => self.occurs(ty),
		BareFn(ty) => self.occurs(ty),
		Group(ty) => self.occurs(ty),
		ImplTrait(ty) => self.occurs(ty),
		Paren(ty) => self.occurs(ty),
		Path(ty) => self.occurs(ty),
		Ptr(ty) => self.occurs(ty),
		Reference(ty) => self.occurs(ty),
		Slice(ty) => self.occurs(ty),
		TraitObject(ty) => self.occurs(ty),
		Tuple(ty) => self.occurs(ty),

		#[cfg(test)]
		Infer(_) |
		Macro(_) |
		Never(_) |
		Verbatim(_) => false,

		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::TypeArray> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeArray) -> bool{
		self.occurs(&tree.elem) || self.occurs(&tree.len)
	}
}

impl Occurs<syn::TypeBareFn> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeBareFn) -> bool{
		!self.occurs(&tree.lifetimes) && (self.occurs(&tree.inputs) || self.occurs(&tree.output))
	}
}

impl Occurs<syn::BoundLifetimes> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::BoundLifetimes) -> bool{
		self.occurs(&tree.lifetimes)
	}
}

impl Occurs<syn::LifetimeDef> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::LifetimeDef) -> bool{
		self.occurs(&tree.lifetime) || self.occurs(&tree.bounds)
	}
}

impl Occurs<syn::BareFnArg> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::BareFnArg) -> bool{
		self.occurs(&tree.ty)
	}
}

impl Occurs<syn::ReturnType> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ReturnType) -> bool{use syn::ReturnType::*; match tree{
		Type(_,ty) => self.occurs(ty),
		_ => false
	}}
}

impl Occurs<syn::TypeGroup> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeGroup) -> bool{
		self.occurs(&tree.elem)
	}
}

impl Occurs<syn::TypeImplTrait> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeImplTrait) -> bool{
		self.occurs(&tree.bounds)
	}
}

impl Occurs<syn::TypeParamBound> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeParamBound) -> bool{use syn::TypeParamBound::*; match tree{
		Trait(bound) => self.occurs(bound),
		Lifetime(bound) => self.occurs(bound),

		#[allow(unreachable_patterns)]
		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::TypeParen> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeParen) -> bool{
		self.occurs(&tree.elem)
	}
}

impl Occurs<syn::TypePath> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypePath) -> bool{match tree.qself.as_ref(){
		Some(qself) => self.occurs(qself),
		None => self.occurs(&tree.path)
	}}
}

impl Occurs<syn::TypePtr> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypePtr) -> bool{
		self.occurs(&tree.elem)
	}
}

impl Occurs<syn::TypeReference> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeReference) -> bool{
		self.occurs(&tree.lifetime) || self.occurs(&tree.elem)
	}
}

impl Occurs<syn::TypeSlice> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeSlice) -> bool{
		self.occurs(&tree.elem)
	}
}

impl Occurs<syn::TypeTraitObject> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeTraitObject) -> bool{
		self.occurs(&tree.bounds)
	}
}

impl Occurs<syn::TypeTuple> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TypeTuple) -> bool{
		self.occurs(&tree.elems)
	}
}

impl Occurs<syn::QSelf> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::QSelf) -> bool{
		self.occurs(&tree.ty)
	}
}

impl Occurs<syn::TraitBound> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::TraitBound) -> bool{
		!self.occurs(&tree.lifetimes) && (self.occurs(&tree.path))
	}
}

impl Occurs<syn::PathSegment> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::PathSegment) -> bool{
		self.occurs(&tree.arguments)
	}
}

impl Occurs<syn::PathArguments> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::PathArguments) -> bool{use syn::PathArguments::*; match tree{
		AngleBracketed(args) => self.occurs(args),
		Parenthesized(args) => self.occurs(args),

		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::AngleBracketedGenericArguments> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::AngleBracketedGenericArguments) -> bool{
		self.occurs(&tree.args)
	}
}

impl Occurs<syn::GenericArgument> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::GenericArgument) -> bool{use syn::GenericArgument::*; match tree{
		Lifetime(arg) => self.occurs(arg),
		Type(arg) => self.occurs(arg),
		Const(arg) => self.occurs(arg),
		Binding(arg) => self.occurs(arg),
		Constraint(arg) => self.occurs(arg),

		#[allow(unreachable_patterns)]
		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::ParenthesizedGenericArguments> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ParenthesizedGenericArguments) -> bool{
		self.occurs(&tree.inputs) || self.occurs(&tree.output)
	}
}

impl Occurs<syn::Binding> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::Binding) -> bool{
		self.occurs(&tree.ty)
	}
}

impl Occurs<syn::Constraint> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::Constraint) -> bool{
		self.occurs(&tree.bounds)
	}
}

//Specific syn expr implementations
//(Inductive cases)

impl Occurs<syn::Expr> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::Expr) -> bool{use syn::Expr::*; match tree{
		Array(expr) => self.occurs(expr),
		Binary(expr) => self.occurs(expr),
		Box(expr) => self.occurs(expr),
		Call(expr) => self.occurs(expr),
		Cast(expr) => self.occurs(expr),
		Field(expr) => self.occurs(expr),
		Group(expr) => self.occurs(expr),
		Index(expr) => self.occurs(expr),
		Lit(expr) => self.occurs(expr),
		MethodCall(expr) => self.occurs(expr),
		Paren(expr) => self.occurs(expr),
		Path(expr) => self.occurs(expr),
		Range(expr) => self.occurs(expr),
		Reference(expr) => self.occurs(expr),
		Repeat(expr) => self.occurs(expr),
		Struct(expr) => self.occurs(expr),
		Tuple(expr) => self.occurs(expr),
		Type(expr) => self.occurs(expr),
		Unary(expr) => self.occurs(expr),

		#[cfg(test)]
		Assign(_) |
		AssignOp(_) |
		Async(_) |
		Await(_) |
		Block(_) |
		Break(_) |
		Closure(_) |
		Continue(_) |
		ForLoop(_) |
		If(_) |
		Let(_) |
		Loop(_) |
		Macro(_) |
		Match(_) |
		Return(_) |
		Try(_) |
		TryBlock(_) |
		Unsafe(_) |
		Verbatim(_) |
		While(_) |
		Yield(_) => false,

		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::ExprArray> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprArray) -> bool{
		self.occurs(&tree.elems)
	}
}

impl Occurs<syn::ExprBinary> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprBinary) -> bool{
		self.occurs(&tree.left) || self.occurs(&tree.right)
	}
}

impl Occurs<syn::ExprBox> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprBox) -> bool{
		self.occurs(&tree.expr)
	}
}

impl Occurs<syn::ExprCall> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprCall) -> bool{
		self.occurs(&tree.func) || self.occurs(&tree.args)
	}
}

impl Occurs<syn::ExprCast> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprCast) -> bool{
		self.occurs(&tree.expr) || self.occurs(&tree.ty)
	}
}

impl Occurs<syn::ExprField> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprField) -> bool{
		self.occurs(&tree.base)
	}
}

impl Occurs<syn::ExprGroup> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprGroup) -> bool{
		self.occurs(&tree.expr)
	}
}

impl Occurs<syn::ExprIndex> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprIndex) -> bool{
		self.occurs(&tree.expr) || self.occurs(&tree.index)
	}
}

impl Occurs<syn::ExprLit> for syn::Ident{
	#[inline] fn occurs(&self,_: &syn::ExprLit) -> bool{
		false
	}
}

impl Occurs<syn::ExprMethodCall> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprMethodCall) -> bool{
		self.occurs(&tree.receiver) || self.occurs(&tree.method) || self.occurs(&tree.turbofish) || self.occurs(&tree.args)
	}
}

impl Occurs<syn::MethodTurbofish> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::MethodTurbofish) -> bool{
		self.occurs(&tree.args)
	}
}

impl Occurs<syn::GenericMethodArgument> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::GenericMethodArgument) -> bool{use syn::GenericMethodArgument::*; match tree{
		Type(arg) => self.occurs(arg),
		Const(arg) => self.occurs(arg),

		#[allow(unreachable_patterns)]
		#[cfg_attr(test,warn(non_exhaustive_omitted_patterns))]
		_ => false
	}}
}

impl Occurs<syn::ExprParen> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprParen) -> bool{
		self.occurs(&tree.expr)
	}
}

impl Occurs<syn::ExprPath> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprPath) -> bool{match tree.qself.as_ref(){
		Some(qself) => self.occurs(qself),
		None => self.occurs(&tree.path)
	}}
}

impl Occurs<syn::ExprRange> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprRange) -> bool{
		self.occurs(&tree.from) || self.occurs(&tree.to)
	}
}

impl Occurs<syn::ExprReference> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprReference) -> bool{
		self.occurs(&tree.expr)
	}
}

impl Occurs<syn::ExprRepeat> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprRepeat) -> bool{
		self.occurs(&tree.expr) || self.occurs(&tree.len)
	}
}

impl Occurs<syn::ExprStruct> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprStruct) -> bool{
		self.occurs(&tree.path) || self.occurs(&tree.fields) || self.occurs(&tree.rest)
	}
}

impl Occurs<syn::FieldValue> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::FieldValue) -> bool{
		self.occurs(&tree.expr)
	}
}

impl Occurs<syn::ExprTuple> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprTuple) -> bool{
		self.occurs(&tree.elems)
	}
}

impl Occurs<syn::ExprType> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprType) -> bool{
		self.occurs(&tree.expr) || self.occurs(&tree.ty)
	}
}

impl Occurs<syn::ExprUnary> for syn::Ident{
	#[inline] fn occurs(&self,tree: &syn::ExprUnary) -> bool{
		self.occurs(&tree.expr)
	}
}
