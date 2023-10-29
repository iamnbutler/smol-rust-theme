```rust
use crate::theme_system::hsla::Hsla;

/// A ColorScale represents the color range in an array of 12 Hsla colors.
/// Each unit has a role:
/// 1. App background
/// 2. Subtle background
/// 3. UI element
/// 4. Hovered UI
/// 5. Active / Selected UI
/// 6. Subtle borders
/// 7. UI element border
/// 8. Hovered UI border
/// 9. Solid backgrounds
/// 10. Hovered solid backgrounds
/// 11. Low-contrast text
/// 12. High-contrast text
pub struct ColorScale {
    pub app_background: Hsla,
    pub subtle_background: Hsla,
    pub ui_element: Hsla,
    pub hovered_ui: Hsla,
    pub active_ui: Hsla,
    pub subtle_borders: Hsla,
    pub ui_element_border: Hsla,
    pub hovered_ui_border: Hsla,
    pub solid_backgrounds: Hsla,
    pub hovered_solid_backgrounds: Hsla,
    pub low_contrast_text: Hsla,
    pub high_contrast_text: Hsla,
}

impl ColorScale {
    /// Create a new ColorScale
    pub fn new(
        app_background: Hsla,
        subtle_background: Hsla,
        ui_element: Hsla,
        hovered_ui: Hsla,
        active_ui: Hsla,
        subtle_borders: Hsla,
        ui_element_border: Hsla,
        hovered_ui_border: Hsla,
        solid_backgrounds: Hsla,
        hovered_solid_backgrounds: Hsla,
        low_contrast_text: Hsla,
        high_contrast_text: Hsla,
    ) -> Self {
        Self {
            app_background,
            subtle_background,
            ui_element,
            hovered_ui,
            active_ui,
            subtle_borders,
            ui_element_border,
            hovered_ui_border,
            solid_backgrounds,
            hovered_solid_backgrounds,
            low_contrast_text,
            high_contrast_text,
        }
    }
}
```