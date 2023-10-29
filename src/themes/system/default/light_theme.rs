use crate::utils::hsla::Hsla;
use crate::utils::ui_color::UIColor;
use crate::utils::color_scale::ColorScale;
use crate::utils::theme::Theme;
use std::collections::BTreeMap;

pub fn light_theme() -> Theme {
    let mut ui_colors: BTreeMap<String, UIColor> = BTreeMap::new();

    ui_colors.insert(
        "background".to_string(),
        UIColor {
            name: "background".to_string(),
            value: ColorScale::new([
                Hsla::hsla(0.0, 0.0, 1.0, 1.0), // white
                Hsla::hsla(0.0, 0.0, 0.9, 1.0), // light gray
                Hsla::hsla(0.0, 0.0, 0.8, 1.0), // gray
                Hsla::hsla(0.0, 0.0, 0.7, 1.0), // dark gray
                Hsla::hsla(0.0, 0.0, 0.6, 1.0), // darker gray
                Hsla::hsla(0.0, 0.0, 0.5, 1.0), // even darker gray
                Hsla::hsla(0.0, 0.0, 0.4, 1.0), // even darker gray
                Hsla::hsla(0.0, 0.0, 0.3, 1.0), // even darker gray
                Hsla::hsla(0.0, 0.0, 0.2, 1.0), // even darker gray
                Hsla::hsla(0.0, 0.0, 0.1, 1.0), // almost black
                Hsla::hsla(0.0, 0.0, 0.0, 1.0), // black
                Hsla::hsla(0.0, 0.0, 0.0, 0.0), // transparent
            ]),
            description: "Background color".to_string(),
        },
    );

    // Add more UI colors as needed...

    Theme {
        name: "light".to_string(),
        ui_colors,
    }
}