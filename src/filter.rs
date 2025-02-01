/// Filter allow users to define rules to filter items produced by
/// the source.
pub trait Filter<T> {
    /// true if it should keep the item
    /// false if should remove the item
    fn filter(&self, item: &T) -> bool;
}
