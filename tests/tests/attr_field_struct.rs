#![allow(unused)]

use enum_traits_macros::*;

#[test]
fn empty(){
	#[transform_enum_field_struct]
	enum X{}
}

#[test]
fn fields(){
	#[transform_enum_field_struct]
	enum Fields<'a,X,Y>{
		A(i8),
		B(i32),
		C(u8,u16,u32),
		D{d: (u8,i32)},
		E{a: i32,b: i32,c: i32,d: i32,e: (u16,i32)},
		F,
		G(X),
		H(&'a Y),
		I{x: X,y: &'a Y},
		J{x: i8},
	}

	let a: A = A(0i8);
	let b: B = B(0i32);
	let c: C = C(1,2,3);
	let d: D = D{d: (0,0)};
	let e: E = E{a: 0,b: 0,c: 0,d: 0,e: (0,0)};
	let f: F = F;
	let g: G<u64> = G(5);
	let h: H<'_,i64> = H(&5);
	let i: I<'_,u64,i64> = I{x: 7,y: &53};
	let j: J = J{x: 1};

	let _: Fields<'_,u64,i64> = Fields::A(a);
	let _: Fields<'_,u64,i64> = Fields::B(b);
	let _: Fields<'_,u64,i64> = Fields::C(c);
	let _: Fields<'_,u64,i64> = Fields::D(d);
	let _: Fields<'_,u64,i64> = Fields::E(e);
	let _: Fields<'_,u64,i64> = Fields::F(f);
	let _: Fields<'_,u64,i64> = Fields::G(g);
	let _: Fields<'_,u64,i64> = Fields::H(h);
	let _: Fields<'_,u64,i64> = Fields::I(i);
	let _: Fields<'_,u64,i64> = Fields::J(j);
}

//Enum has three params but variants uses only one => struct gets only that.
#[test]
fn pruned_generics(){
	#[transform_enum_field_struct]
	enum X<'a,T,U>{
		OnlyT(T),
		OnlyU(U),
		OnlyLife(&'a T),
		NoParam,
	}
	let _: OnlyT<u8> = OnlyT(1u8);
	let _: OnlyU<u16> = OnlyU(2u16);
	let _: OnlyLife<'_,u8> = OnlyLife(&3);
	let _: NoParam = NoParam;
	let _ = X::<u8,u16>::OnlyT(OnlyT(1u8));
	let _ = X::<u8,u16>::NoParam(NoParam);
}

//X of params in the generated struct follows the enum's declaration, not the order they first appear in the fields.
#[test]
fn param_order_is_declaration_order(){
	#[transform_enum_field_struct]
	enum X<'a,T,U>{
		Reordered(U,T,&'a str),
	}
	let s = "x";
	let r: Reordered<'_,u8,u16> = Reordered(1u16,2u8,&s);
	let _: X<'_,u8,u16> = X::Reordered(r);
}

#[test]
fn const_generics(){
	#[transform_enum_field_struct]
	enum X<const N: usize,T>{
		Arr([u8; N]),
		Val(T),
		Both([u8; N],T),
	}
	let a: Arr<3> = Arr([0u8; 3]);
	let v: Val<u8> = Val(1);
	let b: Both<3,u8> = Both([0; 3],1);
	let _ = X::<3,u8>::Arr(a);
	let _ = X::<3,u8>::Val(v);
	let _ = X::<3,u8>::Both(b);
}

#[test]
fn where_bounds(){
	struct S;

	#[transform_enum_field_struct]
	enum X<T> where T: Copy{
		V(T),
	}

	let _: V<S> = V(S); //The where bound is not copied. TODO: ...but maybe it should be?
}

#[test]
fn param_bounds(){
	struct S;

	#[transform_enum_field_struct]
	enum X<T: ?Sized + 'static>{
		V(&'static T),
	}

	let _: V<[u8]> = V(&[1,2]); //The param bounds are copied.
}

#[test]
fn phantom_data(){
	use core::marker::PhantomData;

	#[transform_enum_field_struct]
	enum X<T,U>{
		A(T,PhantomData<U>),
		B(PhantomData<T>),
	}
	let a: A<u8,u16> = A(1u8,PhantomData);
	let b: B<u8> = B(PhantomData);
	let _ = X::<u8,u16>::A(a);
	let _ = X::<u8,u16>::B(b);
}

#[test]
fn rename(){
	#[transform_enum_field_struct]
	enum Renamed{
		#[enum_field_struct(name(Rep))]
		Original(i8),
	}
	let r: Rep = Rep(7i8);
	let _ = Renamed::Original(r);
}

#[test]
fn vis_rename(){
	mod inner{
		use super::*;

		#[transform_enum_field_struct]
		pub enum E{
			#[enum_field_struct(name(pub RenamedPub))]
			A(pub i8),
		}
	}
	let _ = inner::E::A(inner::RenamedPub(1));
}

#[test]
fn attr_rename(){
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(name(#[derive(Debug,Clone,PartialEq,Eq)] Renamed))]
		A(i8),
	}
	let _ = X::A(Renamed(5i8));
}

#[test]
fn attrs_and_visibility_together(){
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(name(#[derive(Debug)] pub Renamed))]
		A(i8),
	}
	let _ = X::A(Renamed(1));
}

#[test]
fn rename_record_variant(){
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(name(MyRec))]
		A{x: i8,y: i16},
	}
	let r = MyRec{x: 1,y: 2};
	let _ = X::A(r);
}

#[test]
fn rename_unit_variant(){
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(name(UnitVar))]
		A,
	}
	let v: UnitVar = UnitVar;
	let _ = X::A(v);
}

#[test]
fn excluded_variant_keeps_fields(){
	#[transform_enum_field_struct]
	enum E{
		#[enum_field_struct(exclude)]
		Kept(i8,i16),
		Wrapped(u8,u16),
	}
	let _ = E::Kept(1i8,2i16);
	let _ = E::Wrapped(Wrapped(1u8,2u16));
}

#[test]
fn excluded_record_variant(){
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(exclude)]
		Kept{x: i8},
		Wrapped{x: i8},
	}
	let _ = X::Kept{x: 1};
	let _ = X::Wrapped(Wrapped{x: 1});
}

#[test]
fn excluded_unit_variant(){
	#[transform_enum_field_struct]
	enum U{
		#[enum_field_struct(exclude)]
		Kept,
		Wrapped,
	}
	let _ = U::Kept;
	let _ = U::Wrapped(Wrapped);
}

#[test]
fn exclude_prevents_struct_gen(){
	struct A;
	#[transform_enum_field_struct]
	enum X{
		#[enum_field_struct(exclude)]
		A(i8),
		B(u8),
	}
	let _ = X::A(1);
	let _ = X::B(B(2));
}

#[test]
fn raw_ident(){
	#![allow(non_camel_case_types,nonstandard_style)]

	#[transform_enum_field_struct]
	enum X{r#type(i8)}
	let _ = X::r#type(r#type(1));
}

#[test]
fn zero_sized_field(){
	#[transform_enum_field_struct]
	enum X{A(())}
    let _ = X::A(A(()));
}
