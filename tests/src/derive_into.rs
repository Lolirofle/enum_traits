#![allow(unused)]

///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// struct S{a: u8}
///```
fn when_struct(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// enum E{
/// 	A(u8,u16),
/// 	B(u16,u8),
/// 	Unit,
/// }
///```
fn unit_variant_last(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// enum E{
/// 	A(u8,u16),
/// 	Unit,
/// 	B(u16,u8),
/// }
///```
fn unit_variant_middle(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// enum E {
///     A(u8),
///     B(u16),
///     C(u32),
/// }
///```
fn no_common_type(){}

///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// enum E {}
///```
fn empty(){}

/* TODO: Maybe this case should give an error?
///```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumInto)]
/// enum E{
/// 	A(u8,u8),
/// 	B(u8,u8),
/// }
///```
fn duplicate_type(){}
*/

///```compile_fail
/// use enum_traits_macros::*;
/// type Alias = u8;
/// #[derive(EnumInto)]
/// enum E {
/// 	A(u8, u16),
/// 	B(Alias, u16),
/// }
/// let _: u8 = E::A(1,2).into();
///```
fn alias_collide1(){}

///```compile_fail
/// use enum_traits_macros::*;
/// type Alias = u8;
/// #[derive(EnumInto)]
/// enum E {
/// 	A(u8, u16),
/// 	B(Alias, u16),
/// }
/// let _: Alias = E::B(1,2).into();
///```
fn alias_collide2(){}

///```compile_fail
/// use enum_traits_macros::*;
/// type Alias = u8;
/// #[derive(EnumInto)]
/// enum E {
/// 	A(u8,Alias),
/// 	B(Alias,u16,u8),
/// }
///```
fn alias_collide(){}
