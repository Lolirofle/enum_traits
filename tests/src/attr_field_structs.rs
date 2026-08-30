///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// struct S{a: u8}
///```
fn on_struct(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// union U{a: u8}
///```
fn on_union(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// fn f(){}
///```
fn on_function(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name)] A(i8),
///}
///```
fn name_without_parens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name())] A(i8),
///}
///```
fn name_empty_parens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(pub))] A(i8),
///}
///```
fn name_vis_without_ident(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(42))] A(i8),
///}
///```
fn name_with_literal(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(Foo) EXTRA)] A(i8),
///}
///```
fn name_trailing_tokens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(pub(invalid::) Foo))] A(i8),
///}
///```
fn name_bad_visibility(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(unknown_option)] A(i8),
///}
///```
fn unknown_option(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(rename(Foo))] A(i8),
///}
///```
fn wrong_option_keyword(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(Foo), name(Bar))] A(i8),
///}
///```
fn duplicate_name(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(exclude, exclude)] A(i8),
///}
///```
fn duplicate_exclude(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(exclude EXTRA)] A(i8),
///}
///```
fn exclude_trailing_tokens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(exclude name(Foo))] A(i8),
///}
///```
fn missing_comma(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(, exclude)] A(i8),
///}
///```
fn leading_comma(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// #[enum_field_struct(exclude)]
/// enum E{A(i8)}
///```
fn attr_on_enum(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	A(#[enum_field_struct(exclude)] i8),
///}
///```
fn attr_on_field(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(exclude)] A(i8),
///}
/// fn main(){let _ = A(1);}
///```
fn excluded_struct_missing(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(Renamed))] Original(i8),
///}
/// fn main(){let _ = Original(1);}
///```
fn pre_rename_name_missing(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(Same))] A(i8),
/// 	#[enum_field_struct(name(Same))] B(i16),
///}
///```
fn two_variants_same_struct_name(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_struct]
/// enum E{
/// 	A(i8),
/// 	#[enum_field_struct(name(A))] B(i16),
///}
///```
fn renamed_collides_with_default(){}

///```compile_fail
/// use enum_traits_macros::*;
/// struct A;
/// #[transform_enum_field_struct]
/// enum E{
/// 	#[enum_field_struct(name(A))] A(i8),
///}
///```
fn renamed_collides_with_existing(){}

/// ```compile_fail
/// mod inner{
/// 	use enum_traits_macros::*;
///
/// 	#[transform_enum_field_struct]
/// 	pub enum X{
/// 		A(u8),
/// 		B{x: u16},
/// 	}
/// }
/// inner::A(1);
/// ```
fn fields_vis1(){}

/// ```compile_fail
/// mod inner{
/// 	use enum_traits_macros::*;
///
/// 	#[transform_enum_field_struct]
/// 	pub enum X{
/// 		A(u8),
/// 		B{x: u16},
/// 	}
/// }
/// inner::B{x: 2};
/// ```
fn fields_vis2(){}
