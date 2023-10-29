```rust
use std::fs;
use std::path::Path;
use theme_system::{ThemeRegistry, SystemThemeFamily};

fn main() {
    let mut theme_registry = ThemeRegistry::new();

    // Load system themes
    let system_theme_family = SystemThemeFamily::default();
    theme_registry.add_theme_family(system_theme_family);

    // Load user themes
    let user_themes_path = Path::new("themes/user.toml");
    if user_themes_path.exists() {
        let user_themes_data = fs::read_to_string(user_themes_path)
            .expect("Failed to read user themes file");
        let user_theme_family = ThemeFamily::from_toml(&user_themes_data);
        theme_registry.add_theme_family(user_theme_family);
    }

    // Display themes in the registry & their details
    theme_registry.display_themes();

    // Alphabetically list all light/dark themes
    theme_registry.list_themes_alphabetically();
}
```