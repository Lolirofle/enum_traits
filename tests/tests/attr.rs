#[test] fn simple(){
	use enum_traits_macros::*;

	#[allow(dead_code)]
	#[impl_enum_len(LENGTH_NAME)]
	#[impl_enum_first(FIRST_NAME)]
	#[impl_enum_last(LAST_NAME)]
	#[impl_enum_variants_array(VARIANTSSS)]
	#[derive(Eq,PartialEq,Debug)]
	enum T{A,B,C,D}

	assert_eq!(T::LENGTH_NAME,4);
	assert_eq!(T::FIRST_NAME,T::A);
	assert_eq!(T::LAST_NAME,T::D);
	assert_eq!(T::VARIANTSSS.len(),4);
	assert_eq!(T::VARIANTSSS,[T::A,T::B,T::C,T::D]);
}

#[test] fn visibility(){
	mod inner{
		use enum_traits_macros::*;

		#[allow(dead_code)]
		#[impl_enum_len(pub(crate) LENGTH)]
		#[derive(Eq,PartialEq,Debug)]
		pub enum T{A,B,C,D}
	}

	assert_eq!(inner::T::LENGTH,4);
}
