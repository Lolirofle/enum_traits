///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A(u8, u16),
///    B{x: u8, y: u16},
///}
///```
fn mixed(){}

///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A,
///    B,
///}
///```
fn two_units(){}

///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A(u8),
///    B(u8),
///}
///```
fn two_tuples(){}

///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A{a: u8},
///    B{b: u8},
///}
///```
fn two_structs(){}

///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A,
///    B(()),
///}
///```
fn unit_unit(){}

///```compile_fail
///use enum_traits_macros::*;
///#[derive(EnumFrom)]
///enum E{
///    A((u8,u16)),
///    B(u8,u16),
///}
///```
fn tuple_nesting(){}
