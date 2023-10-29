```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_scale::Hsla;
    use crate::color_scale_set::ColorScaleSet;

    #[test]
    fn test_color_scale_set_creation() {
        let light_scale = [Hsla::default(); 12];
        let dark_scale = [Hsla::default(); 12];
        let light_alpha_scale = [Hsla::default(); 12];
        let dark_alpha_scale = [Hsla::default(); 12];

        let color_scale_set = ColorScaleSet {
            name: String::from("test"),
            light: light_scale,
            dark: dark_scale,
            light_alpha: light_alpha_scale,
            dark_alpha: dark_alpha_scale,
        };

        assert_eq!(color_scale_set.name, "test");
        assert_eq!(color_scale_set.light, light_scale);
        assert_eq!(color_scale_set.dark, dark_scale);
        assert_eq!(color_scale_set.light_alpha, light_alpha_scale);
        assert_eq!(color_scale_set.dark_alpha, dark_alpha_scale);
    }
}
```