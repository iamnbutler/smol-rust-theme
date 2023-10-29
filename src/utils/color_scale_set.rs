use crate::utils::hsla::Hsla;
use crate::utils::color_scale::ColorScale;

/// A ColorScaleSet represents a set of color scales.
/// It contains exactly 4 ColorScale: `light`, `dark`, `light_alpha` and `dark_alpha`.
/// The `light` and `dark` scales are used for solid colors.
/// The `light_alpha` and `dark_alpha` scales are used for transparent colors.
#[derive(Debug, Clone)]
pub struct ColorScaleSet {
    pub name: String,
    pub light: ColorScale,
    pub dark: ColorScale,
    pub light_alpha: ColorScale,
    pub dark_alpha: ColorScale,
}

impl ColorScaleSet {
    /// Creates a new ColorScaleSet with the given name and color scales.
    pub fn new(name: String, light: ColorScale, dark: ColorScale, light_alpha: ColorScale, dark_alpha: ColorScale) -> Self {
        Self {
            name,
            light,
            dark,
            light_alpha,
            dark_alpha,
        }
    }

    /// Returns a reference to the color scale with the given name.
    /// If no such color scale exists, returns None.
    pub fn get_scale(&self, name: &str) -> Option<&ColorScale> {
        match name {
            "light" => Some(&self.light),
            "dark" => Some(&self.dark),
            "light_alpha" => Some(&self.light_alpha),
            "dark_alpha" => Some(&self.dark_alpha),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hsla::hsla;

    #[test]
    fn test_color_scale_set() {
        let light = ColorScale::new("light".to_string(), [hsla(0., 0., 0., 0.); 12]);
        let dark = ColorScale::new("dark".to_string(), [hsla(0., 0., 0., 0.); 12]);
        let light_alpha = ColorScale::new("light_alpha".to_string(), [hsla(0., 0., 0., 0.); 12]);
        let dark_alpha = ColorScale::new("dark_alpha".to_string(), [hsla(0., 0., 0., 0.); 12]);

        let set = ColorScaleSet::new("test".to_string(), light, dark, light_alpha, dark_alpha);

        assert_eq!(set.get_scale("light").unwrap().name, "light");
        assert_eq!(set.get_scale("dark").unwrap().name, "dark");
        assert_eq!(set.get_scale("light_alpha").unwrap().name, "light_alpha");
        assert_eq!(set.get_scale("dark_alpha").unwrap().name, "dark_alpha");
        assert!(set.get_scale("nonexistent").is_none());
    }
}
