```rust
use super::super::color_scale_set::ColorScaleSet;
use super::super::color_scale::ColorScale;
use super::super::hsla::Hsla;

#[test]
fn test_color_scale_set_creation() {
    let color_scale_set = ColorScaleSet::new(
        "Test Scale Set",
        ColorScale::default(),
        ColorScale::default(),
        ColorScale::default(),
        ColorScale::default(),
    );

    assert_eq!(color_scale_set.name, "Test Scale Set");
    assert_eq!(color_scale_set.light, ColorScale::default());
    assert_eq!(color_scale_set.dark, ColorScale::default());
    assert_eq!(color_scale_set.light_alpha, ColorScale::default());
    assert_eq!(color_scale_set.dark_alpha, ColorScale::default());
}

#[test]
fn test_color_scale_set_modification() {
    let mut color_scale_set = ColorScaleSet::new(
        "Test Scale Set",
        ColorScale::default(),
        ColorScale::default(),
        ColorScale::default(),
        ColorScale::default(),
    );

    let new_color_scale = ColorScale::new([
        Hsla::new(0.1, 0.2, 0.3, 0.4),
        Hsla::new(0.5, 0.6, 0.7, 0.8),
        Hsla::new(0.9, 1.0, 0.1, 0.2),
        Hsla::new(0.3, 0.4, 0.5, 0.6),
        Hsla::new(0.7, 0.8, 0.9, 1.0),
        Hsla::new(0.1, 0.2, 0.3, 0.4),
        Hsla::new(0.5, 0.6, 0.7, 0.8),
        Hsla::new(0.9, 1.0, 0.1, 0.2),
        Hsla::new(0.3, 0.4, 0.5, 0.6),
        Hsla::new(0.7, 0.8, 0.9, 1.0),
        Hsla::new(0.1, 0.2, 0.3, 0.4),
        Hsla::new(0.5, 0.6, 0.7, 0.8),
    ]);

    color_scale_set.light = new_color_scale.clone();
    color_scale_set.dark = new_color_scale.clone();
    color_scale_set.light_alpha = new_color_scale.clone();
    color_scale_set.dark_alpha = new_color_scale;

    assert_eq!(color_scale_set.light, color_scale_set.dark);
    assert_eq!(color_scale_set.light_alpha, color_scale_set.dark_alpha);
}
```