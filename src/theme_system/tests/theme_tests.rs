```rust
use super::super::theme::Theme;
use super::super::hsla::Hsla;
use super::super::ui_color::UIColor;
use super::super::color_scale::ColorScale;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("Test Theme", "light");
        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.appearance, "light");
    }

    #[test]
    fn test_theme_color_mapping() {
        let mut theme = Theme::new("Test Theme", "light");
        let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
        theme.map_color("Test Element", color.clone());
        assert_eq!(theme.get_color("Test Element"), Some(&color));
    }

    #[test]
    fn test_theme_color_override() {
        let mut theme = Theme::new("Test Theme", "light");
        let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
        theme.map_color("Test Element", color.clone());
        let override_color = UIColor::new("Override Color", Hsla::default(), "Override Description");
        theme.override_color("Test Element", override_color.clone());
        assert_eq!(theme.get_color("Test Element"), Some(&override_color));
    }

    #[test]
    fn test_theme_color_removal() {
        let mut theme = Theme::new("Test Theme", "light");
        let color = UIColor::new("Test Color", Hsla::default(), "Test Description");
        theme.map_color("Test Element", color.clone());
        theme.remove_color("Test Element");
        assert_eq!(theme.get_color("Test Element"), None);
    }

    #[test]
    fn test_theme_color_scale_mapping() {
        let mut theme = Theme::new("Test Theme", "light");
        let color_scale = ColorScale::new(vec![Hsla::default(); 12]);
        theme.map_color_scale("Test Scale", color_scale.clone());
        assert_eq!(theme.get_color_scale("Test Scale"), Some(&color_scale));
    }
}
```