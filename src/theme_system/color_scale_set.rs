```rust
use super::color_scale::ColorScale;
use std::collections::HashMap;

/// A ColorScaleSet contains 4 ColorScale and a name, with documentation explaining each scale value's role.
/// ColorScaleSet will have light, dark, light_alpha, dark_alpha scales.
/// Use the `default` ColorScaleSet with the values: gray, red, green, blue, yellow.
/// Themes can define their own ColorScaleSets.
pub struct ColorScaleSet {
    pub name: String,
    pub light: ColorScale,
    pub dark: ColorScale,
    pub light_alpha: ColorScale,
    pub dark_alpha: ColorScale,
}

impl ColorScaleSet {
    /// Create a new ColorScaleSet
    pub fn new(name: String, light: ColorScale, dark: ColorScale, light_alpha: ColorScale, dark_alpha: ColorScale) -> Self {
        ColorScaleSet {
            name,
            light,
            dark,
            light_alpha,
            dark_alpha,
        }
    }

    /// Get a ColorScale by name
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

/// A collection of ColorScaleSets
pub struct ColorScaleSetCollection {
    sets: HashMap<String, ColorScaleSet>,
}

impl ColorScaleSetCollection {
    /// Create a new ColorScaleSetCollection
    pub fn new() -> Self {
        ColorScaleSetCollection {
            sets: HashMap::new(),
        }
    }

    /// Add a ColorScaleSet to the collection
    pub fn add_set(&mut self, set: ColorScaleSet) {
        self.sets.insert(set.name.clone(), set);
    }

    /// Get a ColorScaleSet by name
    pub fn get_set(&self, name: &str) -> Option<&ColorScaleSet> {
        self.sets.get(name)
    }
}
```