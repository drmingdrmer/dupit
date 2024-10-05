struct Cloneable;
#[automatically_derived]
impl ::core::clone::Clone for Cloneable {
    #[inline]
    fn clone(&self) -> Cloneable {
        Cloneable
    }
}
impl ::dupit::Duplicate for Cloneable {
    fn dup(&self) -> Self {
        use ::dupit::impls::DuplicateImpl;
        (&&::dupit::impls::Wrapper(self)).duplicate_impl()
    }
}
