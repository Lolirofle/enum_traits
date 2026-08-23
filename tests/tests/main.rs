//TODO: Unit tests that should fail

/*
#[test]
fn from_discriminants(){
	use enum_traits::*;
	use enum_traits_macros::*;

	#[derive(Debug,Eq,PartialEq,EnumFromDiscriminant)]enum T{
		A = 5,
		B = 1,
		C = 3,
		D = 7,
		//E = 7,
	}

	impl IntoDiscriminant<u8> for T{
		fn into_discriminant(self) -> u8{
			self as u8
		}
	}

	assert_eq!(Some(T::A),t.next());
}
*/
