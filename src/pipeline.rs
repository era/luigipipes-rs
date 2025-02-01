/// Pipeline with source, filters and sinks.
/// Each item is executed in sequence.
///
/// If any filter returns false, the item is dropped.
pub struct Pipeline<T> {
    source: Box<dyn crate::source::Source<T>>,
    sinks: Vec<Box<dyn crate::sink::Sink<T>>>,
    filters: Vec<Box<dyn crate::filter::Filter<T>>>,
}

#[derive(thiserror::Error, Debug)]
pub enum BuilderError {
    #[error("all pipelines need a source")]
    NoSource,
}

pub struct PipelineBuilder<T> {
    source: Option<Box<dyn crate::source::Source<T>>>,
    sinks: Vec<Box<dyn crate::sink::Sink<T>>>,
    filters: Vec<Box<dyn crate::filter::Filter<T>>>,
}

impl<T> PipelineBuilder<T> {
    pub fn new() -> Self {
        Self {
            source: None,
            sinks: Vec::new(),
            filters: Vec::new(),
        }
    }

    pub fn add_source(mut self, source: Box<dyn crate::source::Source<T>>) -> Self {
        self.source = Some(source);
        self
    }

    pub fn add_sink(mut self, sink: Box<dyn crate::sink::Sink<T>>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn add_filter(mut self, filter: Box<dyn crate::filter::Filter<T>>) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn build(self) -> Result<Pipeline<T>, BuilderError> {
        let source = match self.source {
            Some(source) => source,
            None => return Err(BuilderError::NoSource),
        };

        Ok(Pipeline {
            source: source,
            sinks: self.sinks,
            filters: self.filters,
        })
    }
}

impl<T> Pipeline<T> {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        for item in self.source {
            if self.filters.iter().any(|f| !f.filter(&item)) {
                continue;
            }

            for sink in &self.sinks {
                sink.save(&item)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::Filter;
    use crate::sink::Sink;
    use crate::source::Source;
    use std::sync::Arc;

    struct MockFilter {
        should_pass: bool,
    }

    impl MockFilter {
        fn new(should_pass: bool) -> Self {
            Self { should_pass }
        }
    }

    impl<T> Filter<T> for MockFilter {
        fn filter(&self, _: &T) -> bool {
            self.should_pass
        }
    }

    struct MockSink<T> {
        saved_items: std::cell::RefCell<Vec<T>>,
    }

    impl<T: Clone> MockSink<T> {
        fn new() -> Self {
            Self {
                saved_items: std::cell::RefCell::new(vec![]),
            }
        }

        fn get_saved(&self) -> Vec<T> {
            self.saved_items.borrow().clone()
        }
    }

    impl<T: Clone> Sink<T> for Arc<MockSink<T>> {
        fn save(&self, item: &T) -> Result<(), Box<dyn std::error::Error>> {
            self.saved_items.borrow_mut().push(item.clone());
            Ok(())
        }
    }

    #[test]
    fn test_pipeline_filters_out_items() {
        let source = vec![1, 2, 3, 4, 5];
        let filter = MockFilter::new(false); // Filters out all items
        let sink = Arc::new(MockSink::new());

        let pipeline = Pipeline {
            source: Box::new(source),
            filters: vec![Box::new(filter)],
            sinks: vec![Box::new(sink.clone())],
        };

        let result = pipeline.run();

        assert!(result.is_ok());
        assert!(sink.get_saved().is_empty());
    }

    #[test]
    fn test_pipeline_saves_filtered_items() {
        let mut source = vec![1, 2, 3, 4, 5];
        let filter = MockFilter::new(true); // Allows all items
        let sink = Arc::new(MockSink::new());

        let pipeline = Pipeline {
            source: Box::new(source.clone()),
            filters: vec![Box::new(filter)],
            sinks: vec![Box::new(sink.clone())],
        };

        let result = pipeline.run();
        source.reverse();

        assert!(result.is_ok());
        assert_eq!(sink.get_saved(), source);
    }

    #[test]
    fn test_pipeline_handles_multiple_filters() {
        let mut source = vec![1, 2, 3, 4, 5];
        let filter1 = MockFilter::new(true);
        let filter2 = MockFilter::new(false); // One filter rejects everything
        let sink = Arc::new(MockSink::new());

        let pipeline = Pipeline {
            source: Box::new(source.clone()),
            filters: vec![Box::new(filter1), Box::new(filter2)],
            sinks: vec![Box::new(sink.clone())],
        };

        let result = pipeline.run();
        source.reverse();

        assert!(result.is_ok());
        assert!(sink.get_saved().is_empty());
    }

    #[test]
    fn test_pipeline_handles_multiple_sinks() {
        let mut source = vec![1, 2, 3];
        let filter = MockFilter::new(true);
        let sink1 = Arc::new(MockSink::new());
        let sink2 = Arc::new(MockSink::new());

        let pipeline = Pipeline {
            source: Box::new(source.clone()),
            filters: vec![Box::new(filter)],
            sinks: vec![Box::new(sink1.clone()), Box::new(sink2.clone())],
        };

        let result = pipeline.run();
        source.reverse();

        assert!(result.is_ok());
        assert_eq!(sink1.get_saved(), source);
        assert_eq!(sink2.get_saved(), source);
    }
}
