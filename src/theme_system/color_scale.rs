```rust
use crate::theme_system::hsla::Hsla;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ColorScale {
    pub app_background: Hsla,
    pub subtle_background: Hsla,
    pub ui_element: Hsla,
    pub hovered_ui: Hsla,
    pub active_selected_ui: Hsla,
    pub subtle_borders: Hsla,
    pub ui_element_border: Hsla,
    pub hovered_ui_border: Hsla,
    pub solid_backgrounds: Hsla,
    pub hovered_solid_backgrounds: Hsla,
    pub low_contrast_text: Hsla,
    pub high_contrast_text: Hsla,
}

impl ColorScale {
    pub fn new() -> Self {
        Self {
            app_background: Hsla::default(),
            subtle_background: Hsla::default(),
            ui_element: Hsla::default(),
            hovered_ui: Hsla::default(),
            active_selected_ui: Hsla::default(),
            subtle_borders: Hsla::default(),
            ui_element_border: Hsla::default(),
            hovered_ui_border: Hsla::default(),
            solid_backgrounds: Hsla::default(),
            hovered_solid_backgrounds: Hsla::default(),
            low_contrast_text: Hsla::default(),
            high_contrast_text: Hsla::default(),
        }
    }

    pub fn from_map(map: BTreeMap<String, Hsla>) -> Self {
        Self {
            app_background: map.get("app_background").unwrap_or(&Hsla::default()).clone(),
            subtle_background: map.get("subtle_background").unwrap_or(&Hsla::default()).clone(),
            ui_element: map.get("ui_element").unwrap_or(&Hsla::default()).clone(),
            hovered_ui: map.get("hovered_ui").unwrap_or(&Hsla::default()).clone(),
            active_selected_ui: map.get("active_selected_ui").unwrap_or(&Hsla::default()).clone(),
            subtle_borders: map.get("subtle_borders").unwrap_or(&Hsla::default()).clone(),
            ui_element_border: map.get("ui_element_border").unwrap_or(&Hsla::default()).clone(),
            hovered_ui_border: map.get("hovered_ui_border").unwrap_or(&Hsla::default()).clone(),
            solid_backgrounds: map.get("solid_backgrounds").unwrap_or(&Hsla::default()).clone(),
            hovered_solid_backgrounds: map.get("hovered_solid_backgrounds").unwrap_or(&Hsla::default()).clone(),
            low_contrast_text: map.get("low_contrast_text").unwrap_or(&Hsla::default()).clone(),
            high_contrast_text: map.get("high_contrast_text").unwrap_or(&Hsla::default()).clone(),
        }
    }
}
```