/// Source of your pipeline, every item
/// produced by it will go through all
/// steps of the pipeline.
pub trait Source<T> {
    fn next(&mut self) -> Option<T>;
}

impl<T> Iterator for dyn Source<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.next()
    }
}

impl<T> Source<T> for Vec<T> {
    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

/// Async flavour of Source
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncSource<T> {
    async fn next(&mut self) -> Option<T>;
}
