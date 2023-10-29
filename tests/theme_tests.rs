```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::ui_color::UIColor;
    use crate::color_scale::ColorScale;
    use crate::color_scale_set::ColorScaleSet;
    use crate::ui_colors::UIColors;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new();
        assert!(theme.ui_colors.is_empty());
        assert!(theme.color_scale_sets.is_empty());
    }

    #[test]
    fn test_add_color_scale_set() {
        let mut theme = Theme::new();
        let color_scale_set = ColorScaleSet::new("test", [ColorScale::default(); 4]);
        theme.add_color_scale_set(color_scale_set);
        assert_eq!(theme.color_scale_sets.len(), 1);
    }

    #[test]
    fn test_add_ui_color() {
        let mut theme = Theme::new();
        let ui_color = UIColor::new("test", ColorScale::default()[0], "test color");
        theme.add_ui_color(ui_color);
        assert_eq!(theme.ui_colors.len(), 1);
    }

    #[test]
    fn test_get_ui_color() {
        let mut theme = Theme::new();
        let ui_color = UIColor::new("test", ColorScale::default()[0], "test color");
        theme.add_ui_color(ui_color.clone());
        assert_eq!(theme.get_ui_color("test"), Some(&ui_color));
    }

    #[test]
    fn test_get_color_scale_set() {
        let mut theme = Theme::new();
        let color_scale_set = ColorScaleSet::new("test", [ColorScale::default(); 4]);
        theme.add_color_scale_set(color_scale_set.clone());
        assert_eq!(theme.get_color_scale_set("test"), Some(&color_scale_set));
    }
}
```