```rust
use super::super::theme_registry::ThemeRegistry;
use super::super::theme::Theme;
use super::super::theme_family::ThemeFamily;
use super::super::color_scale_set::ColorScaleSet;
use super::super::color_scale::ColorScale;
use super::super::hsla::Hsla;
use super::super::ui_color::UIColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_system_theme() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_system_theme();
        assert!(theme_registry.system_theme.is_some());
    }

    #[test]
    fn test_load_user_theme() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_user_theme();
        assert!(theme_registry.user_theme.is_some());
    }

    #[test]
    fn test_modify_system_theme() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_system_theme();
        let new_theme = Theme::new("New Theme", "Dark", vec![]);
        theme_registry.modify_system_theme(new_theme);
        assert_eq!(theme_registry.system_theme.unwrap().name, "New Theme");
    }

    #[test]
    fn test_save_user_theme() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_system_theme();
        let new_theme = Theme::new("New User Theme", "Light", vec![]);
        theme_registry.save_user_theme(new_theme);
        assert_eq!(theme_registry.user_theme.unwrap().name, "New User Theme");
    }

    #[test]
    fn test_display_themes_in_registry() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_system_theme();
        theme_registry.load_user_theme();
        let themes = theme_registry.display_themes_in_registry();
        assert!(themes.len() > 0);
    }

    #[test]
    fn test_list_all_light_dark_themes() {
        let mut theme_registry = ThemeRegistry::new();
        theme_registry.load_system_theme();
        theme_registry.load_user_theme();
        let themes = theme_registry.list_all_light_dark_themes();
        assert!(themes.len() > 0);
    }
}
```