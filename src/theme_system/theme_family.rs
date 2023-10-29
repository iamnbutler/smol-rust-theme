```rust
use std::collections::BTreeMap;
use super::color_scale_set::ColorScaleSet;
use super::theme::Theme;

#[derive(Debug, Clone)]
pub struct ThemeFamily {
    pub name: String,
    pub author: Option<String>,
    pub scales: Option<ColorScaleSet>,
    pub overrides: Option<BTreeMap<String, String>>,
    pub themes: Vec<Theme>,
}

impl ThemeFamily {
    pub fn new(name: String, author: Option<String>, scales: Option<ColorScaleSet>, overrides: Option<BTreeMap<String, String>>, themes: Vec<Theme>) -> Self {
        Self {
            name,
            author,
            scales,
            overrides,
            themes,
        }
    }

    pub fn from_toml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let theme_family: ThemeFamily = toml::from_str(&data)?;
        Ok(theme_family)
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn remove_theme(&mut self, theme_name: &str) {
        self.themes.retain(|theme| theme.name != theme_name);
    }

    pub fn get_theme(&self, theme_name: &str) -> Option<&Theme> {
        self.themes.iter().find(|theme| theme.name == theme_name)
    }
}
```