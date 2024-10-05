#![doc = include_str!("lib_readme.md")]

#[allow(unused_imports)]
#[macro_use]
extern crate dupit_derive;

#[doc(hidden)]
pub use dupit_derive::Duplicate;

#[doc(hidden)]
pub mod impls {

    /// A wrapper type to allow implementing `DuplicateImpl` for references.
    ///
    /// This wrapper is used to implement [`DuplicateImpl`].
    /// The inner value is a reference to the value to be duplicated,
    /// and should be either `Copy` or `Clone`.
    ///
    /// The specialization is implemented with the Rust autoref feature:
    ///
    /// - <https://github.com/drmingdrmer/tips/blob/main/tips/rust/Rust%20%E5%88%A9%E7%94%A8%20autoref%20%E5%AE%9E%E7%8E%B0%20specialization.md>
    /// - <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>
    /// - <https://github.com/dtolnay/case-studies/tree/master/autoref-specialization#realistic-application>
    pub struct Wrapper<T>(pub T);

    /// A trait for duplicating a value via `Copy` if possible, otherwise via `Clone`.
    pub trait DuplicateImpl<T> {
        fn duplicate_impl(&self) -> T;
    }

    impl<T: Clone> DuplicateImpl<T> for Wrapper<&T> {
        #[inline(always)]
        fn duplicate_impl(&self) -> T {
            self.0.clone()
        }
    }

    impl<T: Copy> DuplicateImpl<T> for &Wrapper<&T> {
        #[inline(always)]
        fn duplicate_impl(&self) -> T {
            *self.0
        }
    }
}

/// A trait for duplicating a value via `Copy` if possible, otherwise via `Clone`.
///
/// This trait is implemented by the `#[derive(Duplicate)]` attribute.
/// Manually implementing this trait is allowed but not recommended.
pub trait Duplicate {
    fn dup(&self) -> Self;
}

/// Copy the `$val_ref` if it implements `Copy`, otherwise clone it.
#[macro_export]
#[doc(hidden)]
macro_rules! copy_or_clone {
    ($val_ref: expr) => {{
        use $crate::impls::DuplicateImpl;
        (&&$crate::impls::Wrapper($val_ref)).duplicate_impl()
    }};
}

#[cfg(test)]
mod tests {
    use crate::Duplicate;

    struct Cloneable(bool);

    impl Clone for Cloneable {
        /// When clone() is called, set the flag to true
        fn clone(&self) -> Self {
            Cloneable(true)
        }
    }

    impl Duplicate for Cloneable {
        fn dup(&self) -> Self {
            copy_or_clone!(self)
        }
    }

    #[derive(Copy)]
    struct Copyable(bool);

    impl Clone for Copyable {
        /// When clone() is called, set the flag to true
        #[allow(clippy::non_canonical_clone_impl)]
        fn clone(&self) -> Self {
            Copyable(true)
        }
    }

    impl Duplicate for Copyable {
        fn dup(&self) -> Self {
            copy_or_clone!(self)
        }
    }

    #[test]
    fn test_foo() {
        let a = Cloneable(false).dup();
        assert!(a.0, "cloned, duplicated via Clone");

        let b = Copyable(false).dup();
        assert!(!b.0, "not cloned duplicated via Copy");
    }
}
