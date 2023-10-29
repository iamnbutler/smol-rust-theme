```rust
use std::collections::BTreeMap;
use super::color_scale::ColorScale;
use super::hsla::Hsla;

#[derive(Debug, Clone)]
pub enum Appearance {
    Light,
    Dark,
    LightAlpha,
    DarkAlpha,
}

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    appearance: Appearance,
    color_map: BTreeMap<String, Hsla>,
    overrides: Option<BTreeMap<String, Hsla>>,
}

impl Theme {
    pub fn new(name: String, appearance: Appearance, color_scale: &ColorScale, overrides: Option<BTreeMap<String, Hsla>>) -> Self {
        let mut color_map = BTreeMap::new();
        color_map.insert("app_background".to_string(), color_scale.app_background);
        color_map.insert("subtle_background".to_string(), color_scale.subtle_background);
        color_map.insert("ui_element".to_string(), color_scale.ui_element);
        color_map.insert("hovered_ui".to_string(), color_scale.hovered_ui);
        color_map.insert("active_selected_ui".to_string(), color_scale.active_selected_ui);
        color_map.insert("subtle_borders".to_string(), color_scale.subtle_borders);
        color_map.insert("ui_element_border".to_string(), color_scale.ui_element_border);
        color_map.insert("hovered_ui_border".to_string(), color_scale.hovered_ui_border);
        color_map.insert("solid_backgrounds".to_string(), color_scale.solid_backgrounds);
        color_map.insert("hovered_solid_backgrounds".to_string(), color_scale.hovered_solid_backgrounds);
        color_map.insert("low_contrast_text".to_string(), color_scale.low_contrast_text);
        color_map.insert("high_contrast_text".to_string(), color_scale.high_contrast_text);

        Self {
            name,
            appearance,
            color_map,
            overrides,
        }
    }

    pub fn get_color(&self, name: &str) -> Option<&Hsla> {
        self.color_map.get(name).or_else(|| self.overrides.as_ref()?.get(name))
    }

    pub fn set_color(&mut self, name: String, color: Hsla) {
        if let Some(overrides) = self.overrides.as_mut() {
            overrides.insert(name, color);
        } else {
            let mut new_overrides = BTreeMap::new();
            new_overrides.insert(name, color);
            self.overrides = Some(new_overrides);
        }
    }
}
```