```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_color_creation() {
        let color_scale = [hsla(0.0, 0.0, 0.0, 1.0); 12];
        let ui_color = UIColor {
            name: "filled-element-background".to_string(),
            value: color_scale[4],
            description: "Used for the background of filled elements, like buttons and checkboxes.".to_string(),
        };

        assert_eq!(ui_color.name, "filled-element-background");
        assert_eq!(ui_color.value, color_scale[4]);
        assert_eq!(ui_color.description, "Used for the background of filled elements, like buttons and checkboxes.");
    }

    #[test]
    fn test_ui_color_value() {
        let color_scale = [hsla(0.0, 0.0, 0.0, 1.0); 12];
        let ui_color = UIColor {
            name: "filled-element-background".to_string(),
            value: color_scale[4],
            description: "Used for the background of filled elements, like buttons and checkboxes.".to_string(),
        };

        assert_eq!(ui_color.value, color_scale[4]);
    }
}
```