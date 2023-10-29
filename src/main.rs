```rust
use std::fs;
use std::path::Path;
use crate::themes::{system, user};
use crate::utils::{theme_registry::ThemeRegistry, theme_family::ThemeFamily};

fn main() {
    let mut theme_registry = ThemeRegistry::new();

    // Load system themes
    let system_theme_path = Path::new("src/themes/system");
    if let Ok(entries) = fs::read_dir(system_theme_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let theme_family: ThemeFamily = system::load_theme_family(entry.path());
                theme_registry.add_theme_family(theme_family);
            }
        }
    }

    // Load user themes
    let user_theme_path = Path::new("src/themes/user");
    if let Ok(entries) = fs::read_dir(user_theme_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let theme_family: ThemeFamily = user::load_theme_family(entry.path());
                theme_registry.add_theme_family(theme_family);
            }
        }
    }

    // Display themes in the registry & their details
    theme_registry.display_themes();

    // Alphabetically list all light/dark themes
    theme_registry.list_themes_alphabetically();
}
```