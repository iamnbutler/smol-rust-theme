```rust
use std::collections::BTreeMap;
use crate::ui_color::UIColor;
use crate::color_scale_set::ColorScaleSet;

/// The Theme struct represents a theme in the system.
/// It contains a collection of UIColors and a collection of ColorScaleSets.
pub struct Theme {
    pub ui_colors: BTreeMap<String, UIColor>,
    pub color_scale_sets: BTreeMap<String, ColorScaleSet>,
}

impl Theme {
    /// Creates a new Theme.
    pub fn new() -> Self {
        Theme {
            ui_colors: BTreeMap::new(),
            color_scale_sets: BTreeMap::new(),
        }
    }

    /// Adds a UIColor to the theme.
    pub fn add_ui_color(&mut self, ui_color: UIColor) {
        self.ui_colors.insert(ui_color.name.clone(), ui_color);
    }

    /// Adds a ColorScaleSet to the theme.
    pub fn add_color_scale_set(&mut self, color_scale_set: ColorScaleSet) {
        self.color_scale_sets.insert(color_scale_set.name.clone(), color_scale_set);
    }

    /// Gets a UIColor from the theme.
    pub fn get_ui_color(&self, name: &str) -> Option<&UIColor> {
        self.ui_colors.get(name)
    }

    /// Gets a ColorScaleSet from the theme.
    pub fn get_color_scale_set(&self, name: &str) -> Option<&ColorScaleSet> {
        self.color_scale_sets.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_scale::ColorScale;
    use crate::color_scale::hsla;

    #[test]
    fn test_theme() {
        let mut theme = Theme::new();

        let ui_color = UIColor {
            name: "filled-element-background".to_string(),
            value: ColorScale::new([hsla(0.0, 0.0, 0.0, 1.0); 12]),
            description: "Used for the background of filled elements, like buttons and checkboxes.".to_string(),
        };
        theme.add_ui_color(ui_color);

        let color_scale_set = ColorScaleSet {
            name: "default".to_string(),
            light: ColorScale::new([hsla(1.0, 1.0, 1.0, 1.0); 12]),
            dark: ColorScale::new([hsla(0.0, 0.0, 0.0, 1.0); 12]),
            light_alpha: ColorScale::new([hsla(1.0, 1.0, 1.0, 0.5); 12]),
            dark_alpha: ColorScale::new([hsla(0.0, 0.0, 0.0, 0.5); 12]),
        };
        theme.add_color_scale_set(color_scale_set);

        assert_eq!(theme.ui_colors.len(), 1);
        assert_eq!(theme.color_scale_sets.len(), 1);
    }
}
```