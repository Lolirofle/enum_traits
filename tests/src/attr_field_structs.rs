///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// struct S{a: u8}
///```
fn on_struct(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// union U{a: u8}
///```
fn on_union(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// fn f(){}
///```
fn on_function(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name)] A(i8),
///}
///```
fn name_without_parens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name())] A(i8),
///}
///```
fn name_empty_parens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(pub))] A(i8),
///}
///```
fn name_vis_without_ident(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(42))] A(i8),
///}
///```
fn name_with_literal(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(Foo) EXTRA)] A(i8),
///}
///```
fn name_trailing_tokens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(pub(invalid::) Foo))] A(i8),
///}
///```
fn name_bad_visibility(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(unknown_option)] A(i8),
///}
///```
fn unknown_option(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(rename(Foo))] A(i8),
///}
///```
fn wrong_option_keyword(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(Foo), name(Bar))] A(i8),
///}
///```
fn duplicate_name(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(exclude, exclude)] A(i8),
///}
///```
fn duplicate_exclude(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(exclude EXTRA)] A(i8),
///}
///```
fn exclude_trailing_tokens(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(exclude name(Foo))] A(i8),
///}
///```
fn missing_comma(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(, exclude)] A(i8),
///}
///```
fn leading_comma(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// #[enum_field_structs(exclude)]
/// enum E{A(i8)}
///```
fn attr_on_enum(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	A(#[enum_field_structs(exclude)] i8),
///}
///```
fn attr_on_field(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(exclude)] A(i8),
///}
/// fn main(){let _ = A(1);}
///```
fn excluded_struct_missing(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(Renamed))] Original(i8),
///}
/// fn main(){let _ = Original(1);}
///```
fn pre_rename_name_missing(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(Same))] A(i8),
/// 	#[enum_field_structs(name(Same))] B(i16),
///}
///```
fn two_variants_same_struct_name(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[transform_enum_field_structs]
/// enum E{
/// 	A(i8),
/// 	#[enum_field_structs(name(A))] B(i16),
///}
///```
fn renamed_collides_with_default(){}

///```compile_fail
/// use enum_traits_macros::*;
/// struct A;
/// #[transform_enum_field_structs]
/// enum E{
/// 	#[enum_field_structs(name(A))] A(i8),
///}
///```
fn renamed_collides_with_existing(){}
