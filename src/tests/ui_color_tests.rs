```rust
use super::super::utils::ui_color::UIColor;
use super::super::utils::hsla::Hsla;
use super::super::utils::color_scale::ColorScale;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_color_creation() {
        let hsla = Hsla::new(0.5, 0.5, 0.5, 0.5);
        let ui_color = UIColor::new("TestColor", hsla, "Test Description");
        assert_eq!(ui_color.name, "TestColor");
        assert_eq!(ui_color.value, hsla);
        assert_eq!(ui_color.description, "Test Description");
    }

    #[test]
    fn test_ui_color_from_color_scale() {
        let color_scale = ColorScale::new([
            Hsla::new(0.1, 0.1, 0.1, 0.1),
            Hsla::new(0.2, 0.2, 0.2, 0.2),
            Hsla::new(0.3, 0.3, 0.3, 0.3),
            Hsla::new(0.4, 0.4, 0.4, 0.4),
            Hsla::new(0.5, 0.5, 0.5, 0.5),
            Hsla::new(0.6, 0.6, 0.6, 0.6),
            Hsla::new(0.7, 0.7, 0.7, 0.7),
            Hsla::new(0.8, 0.8, 0.8, 0.8),
            Hsla::new(0.9, 0.9, 0.9, 0.9),
            Hsla::new(1.0, 1.0, 1.0, 1.0),
            Hsla::new(0.0, 0.0, 0.0, 0.0),
            Hsla::new(0.5, 0.5, 0.5, 0.5),
        ]);
        let ui_color = UIColor::from_color_scale("TestColor", &color_scale, 5, "Test Description");
        assert_eq!(ui_color.name, "TestColor");
        assert_eq!(ui_color.value, color_scale.get(5).unwrap());
        assert_eq!(ui_color.description, "Test Description");
    }
}
```