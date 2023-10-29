```rust
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;
use toml;

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

    pub fn load_themes(&self, path: &str) {
        let entries = fs::read_dir(path).expect("Failed to read directory");

        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            if entry.path().extension().unwrap() == "toml" {
                self.load_theme(entry.path());
            }
        }
    }

    fn load_theme(&self, path: Path) {
        let data = fs::read_to_string(path).expect("Failed to read theme file");
        let theme_family: ThemeFamily = toml::from_str(&data).expect("Failed to parse theme file");
        self.themes.write().unwrap().insert(theme_family.name.clone(), theme_family);
    }

    pub fn get_theme(&self, name: &str) -> Option<Theme> {
        let themes = self.themes.read().unwrap();
        themes.get(name).map(|family| family.get_theme())
    }

    pub fn modify_theme(&self, name: &str, theme: Theme) {
        let mut themes = self.themes.write().unwrap();
        if let Some(family) = themes.get_mut(name) {
            family.modify_theme(theme);
        }
    }

    pub fn save_theme(&self, name: &str, path: &str) {
        let themes = self.themes.read().unwrap();
        if let Some(family) = themes.get(name) {
            let data = toml::to_string(family).expect("Failed to serialize theme");
            fs::write(path, data).expect("Failed to write theme file");
        }
    }

    pub fn list_themes(&self) -> Vec<String> {
        let themes = self.themes.read().unwrap();
        themes.keys().cloned().collect()
    }
}
```