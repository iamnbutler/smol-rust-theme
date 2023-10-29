```rust
use super::super::utils::theme::Theme;
use super::super::utils::ui_color::UIColor;
use super::super::utils::hsla::Hsla;
use super::super::utils::color_scale::ColorScale;
use super::super::utils::color_scale_set::ColorScaleSet;

#[test]
fn test_theme_creation() {
    let theme = Theme::new("Test Theme", "light");
    assert_eq!(theme.name, "Test Theme");
    assert_eq!(theme.appearance, "light");
}

#[test]
fn test_theme_add_color() {
    let mut theme = Theme::new("Test Theme", "light");
    let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
    theme.add_color("Test Color", color.clone());
    assert_eq!(theme.colors.get("Test Color"), Some(&color));
}

#[test]
fn test_theme_remove_color() {
    let mut theme = Theme::new("Test Theme", "light");
    let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
    theme.add_color("Test Color", color.clone());
    theme.remove_color("Test Color");
    assert_eq!(theme.colors.get("Test Color"), None);
}

#[test]
fn test_theme_modify_color() {
    let mut theme = Theme::new("Test Theme", "light");
    let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
    theme.add_color("Test Color", color.clone());
    let modified_color = UIColor::new("Test Color", Hsla::new(0.5, 0.5, 0.5, 0.5), "Modified Description");
    theme.modify_color("Test Color", modified_color.clone());
    assert_eq!(theme.colors.get("Test Color"), Some(&modified_color));
}
```