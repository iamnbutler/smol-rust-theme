```rust
use std::collections::BTreeMap;
use std::sync::RwLock;
use super::theme::Theme;
use super::theme_family::ThemeFamily;

pub struct ThemeRegistry {
    themes: RwLock<BTreeMap<String, ThemeFamily>>,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        ThemeRegistry {
            themes: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn add_theme_family(&self, theme_family: ThemeFamily) {
        let mut themes = self.themes.write().unwrap();
        themes.insert(theme_family.name.clone(), theme_family);
    }

    pub fn get_theme_family(&self, name: &str) -> Option<ThemeFamily> {
        let themes = self.themes.read().unwrap();
        themes.get(name).cloned()
    }

    pub fn get_theme(&self, family_name: &str, theme_name: &str) -> Option<Theme> {
        let themes = self.themes.read().unwrap();
        themes.get(family_name).and_then(|family| family.get_theme(theme_name))
    }

    pub fn list_themes(&self) -> Vec<String> {
        let themes = self.themes.read().unwrap();
        themes.keys().cloned().collect()
    }
}
```