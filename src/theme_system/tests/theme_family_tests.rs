```rust
use super::super::theme_family::ThemeFamily;
use super::super::theme::Theme;
use super::super::color_scale_set::ColorScaleSet;
use super::super::color_scale::ColorScale;
use super::super::hsla::Hsla;
use super::super::ui_color::UIColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_family_creation() {
        let theme_family = ThemeFamily::new("Test Family", "Test Author");
        assert_eq!(theme_family.name, "Test Family");
        assert_eq!(theme_family.author, "Test Author");
    }

    #[test]
    fn test_theme_family_add_theme() {
        let mut theme_family = ThemeFamily::new("Test Family", "Test Author");
        let theme = Theme::new("Test Theme");
        theme_family.add_theme(theme);
        assert_eq!(theme_family.themes.len(), 1);
    }

    #[test]
    fn test_theme_family_add_scale_set() {
        let mut theme_family = ThemeFamily::new("Test Family", "Test Author");
        let color_scale_set = ColorScaleSet::new("Test Scale Set", vec![
            ColorScale::new("Test Scale", vec![
                UIColor::new("Test Color", Hsla::new(0.0, 0.0, 0.0, 0.0), "Test Description")
            ])
        ]);
        theme_family.add_scale_set(color_scale_set);
        assert_eq!(theme_family.scale_sets.len(), 1);
    }

    #[test]
    fn test_theme_family_get_theme() {
        let mut theme_family = ThemeFamily::new("Test Family", "Test Author");
        let theme = Theme::new("Test Theme");
        theme_family.add_theme(theme);
        let retrieved_theme = theme_family.get_theme("Test Theme");
        assert!(retrieved_theme.is_some());
    }

    #[test]
    fn test_theme_family_get_scale_set() {
        let mut theme_family = ThemeFamily::new("Test Family", "Test Author");
        let color_scale_set = ColorScaleSet::new("Test Scale Set", vec![
            ColorScale::new("Test Scale", vec![
                UIColor::new("Test Color", Hsla::new(0.0, 0.0, 0.0, 0.0), "Test Description")
            ])
        ]);
        theme_family.add_scale_set(color_scale_set);
        let retrieved_scale_set = theme_family.get_scale_set("Test Scale Set");
        assert!(retrieved_scale_set.is_some());
    }
}
```