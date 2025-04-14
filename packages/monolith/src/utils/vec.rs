//! Some convenience functions for `Vec`s.

/// Trait for mapping `Vec<Option<T>>` to `Vec<T>`, removing any `None`s.
pub trait OnlySome<T> {
    fn only_some(self) -> Vec<T>;
}

impl<T> OnlySome<T> for Vec<Option<T>> {
    /// Maps `Vec<Option<T>>` to `Vec<T>`, removing any `None`s.
    fn only_some(self) -> Vec<T> {
        self
            .into_iter()
            .filter_map(|t| t)
            .collect()
    }
}