```rust
#[cfg(test)]
mod tests {
    use super::Hsla;
    use super::hsla;

    #[test]
    fn test_hsla_clamping() {
        let color = hsla(1.2, 1.2, 1.2, 1.2);
        assert_eq!(color, Hsla { h: 1.0, s: 1.0, l: 1.0, a: 1.0 });

        let color = hsla(-0.2, -0.2, -0.2, -0.2);
        assert_eq!(color, Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.0 });
    }

    #[test]
    fn test_hsla_equality() {
        let color1 = hsla(0.5, 0.5, 0.5, 0.5);
        let color2 = hsla(0.5, 0.5, 0.5, 0.5);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_hsla_inequality() {
        let color1 = hsla(0.5, 0.5, 0.5, 0.5);
        let color2 = hsla(0.6, 0.6, 0.6, 0.6);
        assert_ne!(color1, color2);
    }
}
```