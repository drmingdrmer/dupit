#[derive(Clone, dupit::Duplicate)]
struct Cloneable<'a, 'b, T: Send>
where
    T: Sync,
    'a: 'b,
{
    foo: T,
}
