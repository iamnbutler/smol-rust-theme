pub mod default;

use crate::utils::theme::Theme;
use crate::utils::theme_family::ThemeFamily;

pub fn load_system_themes() -> Vec<ThemeFamily> {
    vec![
        default::get_default_theme_family(),
    ]
}

pub fn get_system_theme(name: &str) -> Option<Theme> {
    match name {
        "default_light" => Some(default::get_light_theme()),
        "default_dark" => Some(default::get_dark_theme()),
        _ => None,
    }
}