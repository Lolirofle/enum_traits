/// ```compile_fail
/// use enum_traits_macros::*;
/// #[derive(EnumIs)]
/// enum Enum{
/// 	#[enum_is(exclude)] A,
/// 	B(u32),
/// 	C{i: u32},
/// }
/// Enum::A.is_a();
/// ```
fn exclude(){}
