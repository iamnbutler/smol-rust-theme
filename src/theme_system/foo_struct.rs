```rust
use std::borrow::Cow;
use super::hsla::Hsla;

pub struct FooStruct<'a> {
    value: Cow<'a, Hsla>,
}
```