use std::error::Error;
/// Sink saves the items produced by the source and kept by the
/// filters.
pub trait Sink<T> {
    fn save(&self, item: &T) -> Result<(), Box<dyn Error>>;
}

/// The async flavour of `Sink<T>`
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncSink<T> {
    async fn save(&self, item: &T) -> Result<(), Box<dyn Error>>;
}
