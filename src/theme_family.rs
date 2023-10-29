```rust
use std::borrow::Cow;
use crate::theme::Theme;

pub struct ThemeFamily<'a> {
    name: Cow<'a, str>,
    author: Cow<'a, str>,
    variant: Vec<Theme<'a>>,
}

impl<'a> ThemeFamily<'a> {
    pub fn new(name: Cow<'a, str>, author: Cow<'a, str>, variant: Vec<Theme<'a>>) -> Self {
        ThemeFamily { name, author, variant }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn variant(&self) -> &Vec<Theme<'a>> {
        &self.variant
    }
}
```