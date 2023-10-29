```rust
use super::super::utils::color_scale::ColorScale;
use super::super::utils::hsla::Hsla;

#[test]
fn test_color_scale_creation() {
    let color_scale = ColorScale::new([
        Hsla::new(0.0, 0.0, 0.0, 0.0),
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
        Hsla::new(1.0, 1.0, 1.0, 1.0),
    ]);

    assert_eq!(color_scale.get(0), Some(&Hsla::new(0.0, 0.0, 0.0, 0.0)));
    assert_eq!(color_scale.get(5), Some(&Hsla::new(0.5, 0.5, 0.5, 0.5)));
    assert_eq!(color_scale.get(11), Some(&Hsla::new(1.0, 1.0, 1.0, 1.0)));
    assert_eq!(color_scale.get(12), None);
}

#[test]
fn test_color_scale_modification() {
    let mut color_scale = ColorScale::new([
        Hsla::new(0.0, 0.0, 0.0, 0.0),
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
        Hsla::new(1.0, 1.0, 1.0, 1.0),
    ]);

    color_scale.set(0, Hsla::new(0.2, 0.2, 0.2, 0.2));
    assert_eq!(color_scale.get(0), Some(&Hsla::new(0.2, 0.2, 0.2, 0.2)));
}
```