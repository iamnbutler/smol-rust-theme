```rust
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;

mod theme;
mod theme_registry;
mod theme_family;
mod theme_color;
mod system_theme_family;
mod macros;

use theme::Theme;
use theme_registry::ThemeRegistry;
use theme_family::ThemeFamily;
use theme_color::ThemeColor;
use system_theme_family::SystemThemeFamily;

fn main() {
    let mut registry = ThemeRegistry::new();

    // Load system theme family
    let system_theme_family = SystemThemeFamily::default();
    registry.add_theme_family(Cow::Borrowed(&system_theme_family));

    // Load sample user theme family
    let sample_user_theme_family = ThemeFamily::from_toml("themes/sample_user_theme_family.toml");
    registry.add_theme_family(Cow::Owned(sample_user_theme_family));

    // Edit system theme
    let mut edited_system_theme = system_theme_family.light_theme().clone();
    edited_system_theme.set_name("Edited Default Theme");
    let edited_system_theme_family = ThemeFamily::new("Edited Default Theme Family", "System", vec![edited_system_theme]);

    // Save edited system theme
    let toml = edited_system_theme_family.to_toml();
    let mut file = File::create("themes/edited_default_theme.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    // Add edited system theme to registry
    registry.add_theme_family(Cow::Owned(edited_system_theme_family));

    // Print all themes
    println!("All themes:");
    for theme_family in registry.theme_families() {
        for theme in theme_family.themes() {
            println!("{}", theme.name());
        }
    }

    // Print all light themes
    println!("\nLight themes:");
    for theme_family in registry.theme_families() {
        for theme in theme_family.themes() {
            if theme.appearance() == "light" {
                println!("{}", theme.name());
            }
        }
    }

    // Print all dark themes
    println!("\nDark themes:");
    for theme_family in registry.theme_families() {
        for theme in theme_family.themes() {
            if theme.appearance() == "dark" {
                println!("{}", theme.name());
            }
        }
    }
}
```