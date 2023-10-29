```rust
use super::color_scale::ColorScale;
use std::collections::BTreeMap;

/// A ColorScaleSet contains 4 ColorScale and a name, with documentation explaining each scale value's role.
/// ColorScaleSet will have light, dark, light_alpha, dark_alpha scales.
/// Use the `default` ColorScaleSet with the values: gray, red, green, blue, yellow.
/// Themes can define their own ColorScaleSets.
pub struct ColorScaleSet {
    name: String,
    scales: BTreeMap<String, ColorScale>,
}

impl ColorScaleSet {
    /// Creates a new ColorScaleSet with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scales: BTreeMap::new(),
        }
    }

    /// Adds a ColorScale to the set.
    pub fn add_scale(&mut self, name: &str, scale: ColorScale) {
        self.scales.insert(name.to_string(), scale);
    }

    /// Returns a reference to the ColorScale with the given name, if it exists.
    pub fn get_scale(&self, name: &str) -> Option<&ColorScale> {
        self.scales.get(name)
    }

    /// Returns a mutable reference to the ColorScale with the given name, if it exists.
    pub fn get_scale_mut(&mut self, name: &str) -> Option<&mut ColorScale> {
        self.scales.get_mut(name)
    }

    /// Returns the name of the ColorScaleSet.
    pub fn name(&self) -> &str {
        &self.name
    }
}
```