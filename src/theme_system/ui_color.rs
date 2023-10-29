```rust
use std::collections::BTreeMap;
use std::borrow::Cow;
use super::hsla::Hsla;

#[derive(Debug, Clone)]
pub struct UIColor {
    name: Cow<'static, str>,
    value: Hsla,
    description: Cow<'static, str>,
}

impl UIColor {
    pub fn new<S>(name: S, value: Hsla, description: S) -> Self 
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            name: name.into(),
            value,
            description: description.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Hsla {
        &self.value
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub type UIColors = BTreeMap<Cow<'static, str>, UIColor>;
```