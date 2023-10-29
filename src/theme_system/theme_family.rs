```rust
use std::collections::BTreeMap;
use super::theme::Theme;
use super::color_scale_set::ColorScaleSet;

#[derive(Debug, Clone)]
pub struct ThemeFamily {
    pub name: String,
    pub author: String,
    pub scales: Option<ColorScaleSet>,
    pub overrides: Option<BTreeMap<String, String>>,
    pub themes: Vec<Theme>,
}

impl ThemeFamily {
    pub fn new(name: String, author: String) -> Self {
        Self {
            name,
            author,
            scales: None,
            overrides: None,
            themes: Vec::new(),
        }
    }

    pub fn with_scales(mut self, scales: ColorScaleSet) -> Self {
        self.scales = Some(scales);
        self
    }

    pub fn with_overrides(mut self, overrides: BTreeMap<String, String>) -> Self {
        self.overrides = Some(overrides);
        self
    }

    pub fn add_theme(mut self, theme: Theme) -> Self {
        self.themes.push(theme);
        self
    }
}
```