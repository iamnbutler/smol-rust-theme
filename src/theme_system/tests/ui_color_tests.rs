```rust
use super::super::ui_color::UIColor;
use super::super::hsla::Hsla;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_color_creation() {
        let color_name = String::from("test_color");
        let color_value = Hsla::new(0.5, 0.5, 0.5, 1.0);
        let color_description = String::from("This is a test color.");

        let ui_color = UIColor::new(color_name.clone(), color_value, color_description.clone());

        assert_eq!(ui_color.name, color_name);
        assert_eq!(ui_color.value, color_value);
        assert_eq!(ui_color.description, color_description);
    }

    #[test]
    fn test_ui_color_editing() {
        let mut ui_color = UIColor::new(String::from("test_color"), Hsla::new(0.5, 0.5, 0.5, 1.0), String::from("This is a test color."));
        let new_color_value = Hsla::new(0.2, 0.2, 0.2, 1.0);

        ui_color.edit_value(new_color_value);

        assert_eq!(ui_color.value, new_color_value);
    }
}
```