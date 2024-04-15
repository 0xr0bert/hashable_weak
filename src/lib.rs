#[warn(missing_docs)]
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Weak;

/// A [Weak] pointer that can be [Hash]ed.
pub struct HashableWeak<T> {
    /// The contained [Weak].
    pub weak: Weak<T>
}

impl<T> PartialEq for HashableWeak<T> {
    /// Two [HashableWeak]s are equal if and only if they have the same pointer address.
    fn eq(&self, other: &Self) -> bool {
        self.weak.ptr_eq(&other.weak)
    }
}

impl<T> Eq for HashableWeak<T> {}

impl<T> Hash for HashableWeak<T> {
    /// Hash the pointer based upon its address.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.weak.as_ptr().hash(state);
    }
}

impl<T> Deref for HashableWeak<T> {
    type Target = Weak<T>;

    /// Dereference the [HashableWeak] into its [Weak].
    fn deref(&self) -> &Self::Target {
        &self.weak
    }
}