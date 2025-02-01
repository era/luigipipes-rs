# Pipeline Library

## Overview

`luigipipes-rs` is a Rust library that allows processing items in a sequence through a defined pipeline. Each item originates from a **source**, is processed through a series of **filters**, and is then passed to one or more **sinks** if it meets the filter criteria.

## Features

- Supports multiple **sources** for item input.
- Provides **filters** to selectively process items.
- Allows multiple **sinks** for storing or using the processed items.
- Implements a **builder pattern** for easy construction.

## Usage



### 1. Define a Pipeline

```rust
use pipeline::{PipelineBuilder, Pipeline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Box::new(MySource::new());
    let filter = Box::new(MyFilter::new());
    let sink = Box::new(MySink::new());
    
    let pipeline = PipelineBuilder::new()
        .add_source(source)
        .add_filter(filter)
        .add_sink(sink)
        .build()?;
    
    pipeline.run()?;
    
    Ok(())
}
```

### 2. Implement Custom Components

#### **Custom Source**

```rust
use pipeline::source::Source;

struct MySource;
impl MySource {
    fn new() -> Self { Self }
}
impl Iterator for MySource {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some("Hello, Pipeline!".to_string())
    }
}
impl Source<String> for MySource {}
```

#### **Custom Filter**

```rust
use pipeline::filter::Filter;

struct MyFilter;
impl MyFilter {
    fn new() -> Self { Self }
}
impl Filter<String> for MyFilter {
    fn filter(&self, item: &String) -> bool {
        !item.is_empty()
    }
}
```

#### **Custom Sink**

```rust
use pipeline::sink::Sink;

struct MySink;
impl MySink {
    fn new() -> Self { Self }
}
impl Sink<String> for MySink {
    fn save(&self, item: &String) -> Result<(), Box<dyn std::error::Error>> {
        println!("Saving: {}", item);
        Ok(())
    }
}
```

## Contributing

Contributions are welcome! Feel free to open issues and submit pull requests.

## License

This project is licensed under the MIT License.
