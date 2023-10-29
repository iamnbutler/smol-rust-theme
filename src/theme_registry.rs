```rust
use std::collections::HashMap;
use std::borrow::Cow;
use crate::theme_family::ThemeFamily;
use crate::theme::Theme;

pub struct ThemeRegistry<'a> {
    theme_families: HashMap<String, Cow<'a, ThemeFamily>>,
}

impl<'a> ThemeRegistry<'a> {
    pub fn new() -> Self {
        ThemeRegistry {
            theme_families: HashMap::new(),
        }
    }

    pub fn add_theme_family(&mut self, theme_family: Cow<'a, ThemeFamily>) {
        self.theme_families.insert(theme_family.name.clone(), theme_family);
    }

    pub fn list_all_theme_families(&self) -> Vec<&str> {
        self.theme_families.keys().map(|k| k.as_str()).collect()
    }

    pub fn list_all_themes(&self) -> Vec<&Theme> {
        self.theme_families.values().flat_map(|tf| tf.variants.iter()).collect()
    }

    pub fn list_all_light_themes(&self) -> Vec<&Theme> {
        self.theme_families.values().flat_map(|tf| tf.variants.iter()).filter(|t| t.appearance == "light").collect()
    }

    pub fn list_all_dark_themes(&self) -> Vec<&Theme> {
        self.theme_families.values().flat_map(|tf| tf.variants.iter()).filter(|t| t.appearance == "dark").collect()
    }
}
```