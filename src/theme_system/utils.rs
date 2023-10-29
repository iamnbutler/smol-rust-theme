```rust
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use toml;

use super::theme::Theme;
use super::theme_family::ThemeFamily;

pub fn load_theme_from_file(file_path: &str) -> Result<ThemeFamily, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let theme_family: ThemeFamily = toml::from_str(&content)?;
    Ok(theme_family)
}

pub fn save_theme_to_file(theme_family: &ThemeFamily, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml = toml::to_string(theme_family)?;
    fs::write(file_path, toml)?;
    Ok(())
}

pub fn list_themes_in_directory(directory_path: &str) -> Result<BTreeMap<String, Theme>, Box<dyn std::error::Error>> {
    let mut themes = BTreeMap::new();

    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let theme_family = load_theme_from_file(path.to_str().unwrap())?;
            for theme in theme_family.themes {
                themes.insert(theme.name.clone(), theme);
            }
        }
    }

    Ok(themes)
}
```