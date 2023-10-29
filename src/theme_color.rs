```rust
use std::borrow::Cow;
use palette::Hsla;

#[derive(Debug, Clone)]
pub struct ThemeColor<'a> {
    name: Cow<'a, str>,
    value: Hsla,
    documentation: Option<Cow<'a, str>>,
}

impl<'a> ThemeColor<'a> {
    pub fn new<S>(name: S, value: Hsla, documentation: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            value,
            documentation: documentation.map(Into::into),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Hsla {
        &self.value
    }

    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}
```