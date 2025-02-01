/// Filter allow users to define rules to filter items produced by
/// the source.
pub trait Filter<T> {
    /// true if it should keep the item
    /// false if should remove the item
    fn filter(&self, item: &T) -> bool;
}

/// The async flavour of `Filter<T>`
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncFilter<T> {
    async fn filter(&self, item: &T) -> bool;
}
