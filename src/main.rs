```rust
mod theme_system;

use theme_system::theme_registry::ThemeRegistry;
use theme_system::theme::Theme;
use theme_system::theme_family::ThemeFamily;
use theme_system::color_scale::ColorScale;
use theme_system::color_scale_set::ColorScaleSet;
use theme_system::hsla::Hsla;
use theme_system::ui_color::UIColor;
use theme_system::macros;

use std::collections::BTreeMap;
use std::borrow::Cow;

fn main() {
    let mut theme_registry = ThemeRegistry::new();

    // Load system and user themes
    theme_registry.load_system_themes();
    theme_registry.load_user_themes();

    // Display themes in the registry & their details
    theme_registry.display_themes();

    // Alphabetically list all light/dark themes
    theme_registry.list_themes_alphabetically();

    // Modify system theme and save as a fresh user theme
    if let Some(system_theme) = theme_registry.get_theme("system") {
        let mut user_theme = system_theme.clone();
        user_theme.set_name("user");
        user_theme.set_color(UIColor::new("brand_color", Hsla::new(0.0, 0.0, 0.0, 1.0), "Brand color"));
        theme_registry.add_theme(user_theme);
        theme_registry.save_user_themes();
    }
}
```