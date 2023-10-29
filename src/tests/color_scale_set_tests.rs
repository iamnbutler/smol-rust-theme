```rust
use super::super::utils::color_scale_set::ColorScaleSet;
use super::super::utils::hsla::Hsla;

#[test]
fn test_color_scale_set_creation() {
    let light_scale = [Hsla::default(); 12];
    let dark_scale = [Hsla::default(); 12];
    let light_alpha_scale = [Hsla::default(); 12];
    let dark_alpha_scale = [Hsla::default(); 12];

    let color_scale_set = ColorScaleSet::new(
        "Test Scale Set",
        light_scale,
        dark_scale,
        light_alpha_scale,
        dark_alpha_scale,
    );

    assert_eq!(color_scale_set.name, "Test Scale Set");
    assert_eq!(color_scale_set.light, light_scale);
    assert_eq!(color_scale_set.dark, dark_scale);
    assert_eq!(color_scale_set.light_alpha, light_alpha_scale);
    assert_eq!(color_scale_set.dark_alpha, dark_alpha_scale);
}

#[test]
fn test_color_scale_set_default() {
    let color_scale_set = ColorScaleSet::default();

    assert_eq!(color_scale_set.name, "Default");
    assert_eq!(color_scale_set.light, [Hsla::default(); 12]);
    assert_eq!(color_scale_set.dark, [Hsla::default(); 12]);
    assert_eq!(color_scale_set.light_alpha, [Hsla::default(); 12]);
    assert_eq!(color_scale_set.dark_alpha, [Hsla::default(); 12]);
}
```