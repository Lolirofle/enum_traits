use enum_traits_macros::*;

///```compile_fail
///mod inner{
///  use enum_traits_macros::*;
///  #[impl_enum_first(FI)] pub enum E{}
///}
///inner::E::FI;
///```
fn vis_first(){}

///```compile_fail
///mod inner{
///  use enum_traits_macros::*;
///  #[impl_enum_last(LA)] pub enum E{}
///}
///inner::E::LA;
///```
fn vis_last(){}

///```no_compile
///tests::attrs::T::FIR;
///```
#[impl_enum_first(#[cfg(all(test,not(test)))] pub FIR)]
pub enum AttrRemovalFirst{A}

///```no_compile
///tests::attrs::T::LAS;
///```
#[impl_enum_last(#[cfg(all(test,not(test)))] pub LAS)]
pub enum AttrRemovalLast{A}

///```no_compile
///#[impl_enum_first(FIR)] enum T{}
///```
fn no_empty_first(){}

///```no_compile
///#[impl_enum_last(LAS)] enum T{}
///```
fn no_empty_last(){}

///```no_compile
///#[impl_enum_first(LAS)] enum T{A(i32),B}
///```
fn tuple_no_first(){}

///```no_compile
///#[impl_enum_last(LAS)] enum T{A,B(i32)}
///```
fn tuple_no_last(){}
