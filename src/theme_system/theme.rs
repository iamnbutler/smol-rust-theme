```rust
use std::collections::BTreeMap;
use std::borrow::Cow;
use super::hsla::Hsla;
use super::ui_color::UIColor;

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    appearance: Appearance,
    color_map: BTreeMap<String, UIColor>,
    overrides: Option<BTreeMap<String, UIColor>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Appearance {
    Light,
    Dark,
}

impl Theme {
    pub fn new(name: &str, appearance: Appearance) -> Self {
        Self {
            name: name.to_string(),
            appearance,
            color_map: BTreeMap::new(),
            overrides: None,
        }
    }

    pub fn with_overrides(name: &str, appearance: Appearance, overrides: BTreeMap<String, UIColor>) -> Self {
        Self {
            name: name.to_string(),
            appearance,
            color_map: BTreeMap::new(),
            overrides: Some(overrides),
        }
    }

    pub fn add_color(&mut self, name: &str, color: UIColor) {
        self.color_map.insert(name.to_string(), color);
    }

    pub fn get_color(&self, name: &str) -> Option<&UIColor> {
        self.color_map.get(name)
    }

    pub fn edit_color(&mut self, name: &str, color: UIColor) {
        if let Some(override_map) = &mut self.overrides {
            override_map.insert(name.to_string(), color);
        } else {
            self.color_map.insert(name.to_string(), color);
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn appearance(&self) -> Appearance {
        self.appearance
    }

    pub fn color_map(&self) -> &BTreeMap<String, UIColor> {
        &self.color_map
    }

    pub fn overrides(&self) -> Option<&BTreeMap<String, UIColor>> {
        self.overrides.as_ref()
    }
}
```