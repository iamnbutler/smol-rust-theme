pub mod one;

use std::fs;
use std::path::Path;
use toml;
use crate::utils::theme_family::ThemeFamily;

pub fn load_user_themes() -> Vec<ThemeFamily> {
    let mut user_themes = Vec::new();
    let paths = fs::read_dir("./themes/user").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let theme_family = load_theme_family(&path);
            user_themes.push(theme_family);
        }
    }

    user_themes
}

fn load_theme_family(path: &Path) -> ThemeFamily {
    let index_path = path.join("index.toml");
    let index_content = fs::read_to_string(index_path).unwrap();
    let mut theme_family: ThemeFamily = toml::from_str(&index_content).unwrap();

    let theme_paths = fs::read_dir(path).unwrap();
    for theme_path in theme_paths {
        let theme_path = theme_path.unwrap().path();
        if theme_path.is_file() && theme_path.extension().unwrap() == "toml" && theme_path.file_stem().unwrap() != "index" {
            let theme_content = fs::read_to_string(theme_path).unwrap();
            let theme: Theme = toml::from_str(&theme_content).unwrap();
            theme_family.themes.push(theme);
        }
    }

    theme_family
}