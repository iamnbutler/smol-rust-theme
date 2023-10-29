```rust
use super::super::utils::theme_family::ThemeFamily;
use super::super::utils::theme::Theme;
use super::super::utils::color_scale_set::ColorScaleSet;
use super::super::utils::color_scale::ColorScale;
use super::super::utils::hsla::Hsla;
use super::super::utils::ui_color::UIColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_family_creation() {
        let theme_family = ThemeFamily::new("TestThemeFamily".to_string());
        assert_eq!(theme_family.name, "TestThemeFamily");
    }

    #[test]
    fn test_theme_family_add_theme() {
        let mut theme_family = ThemeFamily::new("TestThemeFamily".to_string());
        let theme = Theme::new("TestTheme".to_string());
        theme_family.add_theme(theme);
        assert_eq!(theme_family.themes.len(), 1);
    }

    #[test]
    fn test_theme_family_remove_theme() {
        let mut theme_family = ThemeFamily::new("TestThemeFamily".to_string());
        let theme = Theme::new("TestTheme".to_string());
        theme_family.add_theme(theme);
        theme_family.remove_theme("TestTheme".to_string());
        assert_eq!(theme_family.themes.len(), 0);
    }

    #[test]
    fn test_theme_family_add_color_scale_set() {
        let mut theme_family = ThemeFamily::new("TestThemeFamily".to_string());
        let color_scale_set = ColorScaleSet::new("TestColorScaleSet".to_string());
        theme_family.add_color_scale_set(color_scale_set);
        assert_eq!(theme_family.color_scale_sets.len(), 1);
    }

    #[test]
    fn test_theme_family_remove_color_scale_set() {
        let mut theme_family = ThemeFamily::new("TestThemeFamily".to_string());
        let color_scale_set = ColorScaleSet::new("TestColorScaleSet".to_string());
        theme_family.add_color_scale_set(color_scale_set);
        theme_family.remove_color_scale_set("TestColorScaleSet".to_string());
        assert_eq!(theme_family.color_scale_sets.len(), 0);
    }
}
```