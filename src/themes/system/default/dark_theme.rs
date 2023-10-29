use crate::utils::theme::Theme;
use crate::utils::ui_color::UIColor;
use crate::utils::hsla::Hsla;
use std::collections::BTreeMap;

pub fn dark_theme() -> Theme {
    let mut ui_colors: BTreeMap<String, UIColor> = BTreeMap::new();

    ui_colors.insert(
        "background".to_string(),
        UIColor {
            name: "background".to_string(),
            value: Hsla::new(0.0, 0.0, 0.2, 1.0),
            description: "Background color for the dark theme".to_string(),
        },
    );

    ui_colors.insert(
        "foreground".to_string(),
        UIColor {
            name: "foreground".to_string(),
            value: Hsla::new(0.0, 0.0, 0.8, 1.0),
            description: "Foreground color for the dark theme".to_string(),
        },
    );

    Theme {
        name: "dark".to_string(),
        appearance: "dark".to_string(),
        ui_colors,
    }
}