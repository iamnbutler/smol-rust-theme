```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsla_clamping() {
        let color = hsla(1.2, 1.2, 1.2, 1.2);
        assert_eq!(color, Hsla { h: 1.0, s: 1.0, l: 1.0, a: 1.0 });

        let color = hsla(-0.2, -0.2, -0.2, -0.2);
        assert_eq!(color, Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 });
    }

    #[test]
    fn test_color_scale_length() {
        let scale = ColorScale::new();
        assert_eq!(scale.len(), 12);
    }

    #[test]
    fn test_color_scale_light_to_dark() {
        let scale = ColorScale::new();
        for i in 0..11 {
            assert!(scale[i].l <= scale[i + 1].l);
        }
    }

    #[test]
    fn test_color_scale_dark_to_light() {
        let scale = ColorScale::new().reverse();
        for i in 0..11 {
            assert!(scale[i].l >= scale[i + 1].l);
        }
    }
}
```