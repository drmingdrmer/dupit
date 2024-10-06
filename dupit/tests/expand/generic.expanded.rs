struct Cloneable<'a, 'b, T: Send>
where
    T: Sync,
    'a: 'b,
{
    foo: T,
}
#[automatically_derived]
impl<'a, 'b, T: ::core::clone::Clone + Send> ::core::clone::Clone
for Cloneable<'a, 'b, T>
where
    T: Sync,
    'a: 'b,
{
    #[inline]
    fn clone(&self) -> Cloneable<'a, 'b, T> {
        Cloneable {
            foo: ::core::clone::Clone::clone(&self.foo),
        }
    }
}
impl<'a, 'b, T: Send> ::dupit::Duplicate for Cloneable<'a, 'b, T>
where
    T: Sync,
    'a: 'b,
{
    fn dup(&self) -> Self {
        use ::dupit::impls::DuplicateImpl;
        (&&::dupit::impls::Wrapper(self)).duplicate_impl()
    }
}
