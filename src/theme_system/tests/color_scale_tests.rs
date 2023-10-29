```rust
use super::super::color_scale::ColorScale;
use super::super::hsla::Hsla;

#[test]
fn test_color_scale_creation() {
    let color_scale = ColorScale::new([
        Hsla::new(0.0, 0.0, 0.0, 1.0), // App background
        Hsla::new(0.1, 0.1, 0.1, 1.0), // Subtle background
        Hsla::new(0.2, 0.2, 0.2, 1.0), // UI element
        Hsla::new(0.3, 0.3, 0.3, 1.0), // Hovered UI
        Hsla::new(0.4, 0.4, 0.4, 1.0), // Active / Selected UI
        Hsla::new(0.5, 0.5, 0.5, 1.0), // Subtle borders
        Hsla::new(0.6, 0.6, 0.6, 1.0), // UI element border
        Hsla::new(0.7, 0.7, 0.7, 1.0), // Hovered UI border
        Hsla::new(0.8, 0.8, 0.8, 1.0), // Solid backgrounds
        Hsla::new(0.9, 0.9, 0.9, 1.0), // Hovered solid backgrounds
        Hsla::new(1.0, 1.0, 1.0, 0.5), // Low-contrast text
        Hsla::new(1.0, 1.0, 1.0, 1.0), // High-contrast text
    ]);

    assert_eq!(color_scale.get(0), Some(&Hsla::new(0.0, 0.0, 0.0, 1.0)));
    assert_eq!(color_scale.get(11), Some(&Hsla::new(1.0, 1.0, 1.0, 1.0)));
    assert_eq!(color_scale.get(12), None);
}

#[test]
fn test_color_scale_modification() {
    let mut color_scale = ColorScale::new([
        Hsla::new(0.0, 0.0, 0.0, 1.0), // App background
        Hsla::new(0.1, 0.1, 0.1, 1.0), // Subtle background
        Hsla::new(0.2, 0.2, 0.2, 1.0), // UI element
        Hsla::new(0.3, 0.3, 0.3, 1.0), // Hovered UI
        Hsla::new(0.4, 0.4, 0.4, 1.0), // Active / Selected UI
        Hsla::new(0.5, 0.5, 0.5, 1.0), // Subtle borders
        Hsla::new(0.6, 0.6, 0.6, 1.0), // UI element border
        Hsla::new(0.7, 0.7, 0.7, 1.0), // Hovered UI border
        Hsla::new(0.8, 0.8, 0.8, 1.0), // Solid backgrounds
        Hsla::new(0.9, 0.9, 0.9, 1.0), // Hovered solid backgrounds
        Hsla::new(1.0, 1.0, 1.0, 0.5), // Low-contrast text
        Hsla::new(1.0, 1.0, 1.0, 1.0), // High-contrast text
    ]);

    color_scale.set(0, Hsla::new(0.5, 0.5, 0.5, 1.0));
    assert_eq!(color_scale.get(0), Some(&Hsla::new(0.5, 0.5, 0.5, 1.0)));
}
```