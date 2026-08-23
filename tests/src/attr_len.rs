use enum_traits_macros::*;

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len()] enum E{}
///```
fn empty_arg(){}

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len(pub)] enum E{}
///```
fn nameless(){}

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len(pub)] struct E{}
///```
fn not_enum(){}

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len(55)] enum E{}
///```
fn literal_name(){}

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len(NAME EXTRA)] enum E{}
///```
fn extra_token(){}

///```compile_fail
///use enum_traits_macros::*;
///#[impl_enum_len(NAME)] enum E{}
///impl E{const NAME: usize = 5;}
///```
fn duplicate(){}

///```compile_fail
///mod inner{
///  use enum_traits_macros::*;
///  #[impl_enum_len(NAME)] pub enum E{}
///}
///inner::E::NAME;
///```
fn vis(){}

/// ```no_compile
/// tests::attrs::T::LENGTH;
/// ```
#[impl_enum_len(#[cfg(all(test,not(test)))] pub LENGTH)]
pub enum AttrRemoval{}
