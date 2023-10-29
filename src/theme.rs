```rust
use std::borrow::Cow;
use crate::theme_color::ThemeColor;

#[derive(Debug, Clone)]
pub enum Appearance {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct Theme<'a> {
    name: Cow<'a, str>,
    appearance: Appearance,
    colors: Vec<ThemeColor<'a>>,
}

impl<'a> Theme<'a> {
    pub fn new<S>(name: S, appearance: Appearance, colors: Vec<ThemeColor<'a>>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            appearance,
            colors,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn appearance(&self) -> &Appearance {
        &self.appearance
    }

    pub fn colors(&self) -> &Vec<ThemeColor<'a>> {
        &self.colors
    }
}
```