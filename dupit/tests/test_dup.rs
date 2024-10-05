#[derive(dupit::Duplicate)]
struct Cloneable(bool);

impl Clone for Cloneable {
    /// When clone() is called, set the flag to true
    fn clone(&self) -> Self {
        Cloneable(true)
    }
}

#[derive(dupit::Duplicate, Copy)]
struct Copyable(bool);

impl Clone for Copyable {
    /// When clone() is called, set the flag to true
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self {
        Copyable(true)
    }
}

#[test]
fn test_copy_clone() {
    use dupit::Duplicate;

    let a = Cloneable(false).dup();
    assert!(a.0, "cloned, duplicated via Clone");

    let b = Copyable(false).dup();
    assert!(!b.0, "not cloned duplicated via Copy");
}
