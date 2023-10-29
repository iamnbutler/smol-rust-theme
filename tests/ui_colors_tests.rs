```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_ui_colors() {
        let mut ui_colors = BTreeMap::new();
        let color_scale = [
            hsla(0.0, 0.0, 0.0, 1.0),
            hsla(0.0, 0.0, 0.05, 1.0),
            hsla(0.0, 0.0, 0.1, 1.0),
            hsla(0.0, 0.0, 0.15, 1.0),
            hsla(0.0, 0.0, 0.2, 1.0),
            hsla(0.0, 0.0, 0.25, 1.0),
            hsla(0.0, 0.0, 0.3, 1.0),
            hsla(0.0, 0.0, 0.35, 1.0),
            hsla(0.0, 0.0, 0.4, 1.0),
            hsla(0.0, 0.0, 0.45, 1.0),
            hsla(0.0, 0.0, 0.5, 1.0),
            hsla(0.0, 0.0, 0.55, 1.0),
        ];

        ui_colors.insert(
            "filled-element-background",
            UIColor {
                name: "filled-element-background",
                value: color_scale[4],
                description: "Used for the background of filled elements, like buttons and checkboxes.",
            },
        );

        assert_eq!(
            ui_colors.get("filled-element-background").unwrap().value,
            color_scale[4]
        );
    }
}
```