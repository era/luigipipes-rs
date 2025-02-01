use std::error::Error;
/// Sink saves the items produced by the source and kept by the
/// filters.
pub trait Sink<T> {
    fn save(&self, item: &T) -> Result<(), Box<dyn Error>>;
}
