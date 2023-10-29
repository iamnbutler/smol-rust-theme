```rust
use std::borrow::Cow;
use crate::theme::Theme;
use crate::theme_family::ThemeFamily;
use crate::theme_color::ThemeColor;
use crate::theme_registry::ThemeRegistry;
use crate::Hsla;

pub struct SystemThemeFamily<'a> {
    name: Cow<'a, str>,
    author: Cow<'a, str>,
    variants: Vec<Theme<'a>>,
}

impl<'a> SystemThemeFamily<'a> {
    pub fn new() -> Self {
        let light_theme = Theme {
            name: Cow::Borrowed("Default Light"),
            appearance: Cow::Borrowed("Light"),
            colors: vec![
                ThemeColor {
                    name: Cow::Borrowed("Background"),
                    value: Hsla::new(0.0, 0.0, 100.0, 1.0),
                    documentation: Some(Cow::Borrowed("The background color of the light theme")),
                },
                // Add more colors as needed
            ],
        };

        let dark_theme = Theme {
            name: Cow::Borrowed("Default Dark"),
            appearance: Cow::Borrowed("Dark"),
            colors: vec![
                ThemeColor {
                    name: Cow::Borrowed("Background"),
                    value: Hsla::new(0.0, 0.0, 0.0, 1.0),
                    documentation: Some(Cow::Borrowed("The background color of the dark theme")),
                },
                // Add more colors as needed
            ],
        };

        Self {
            name: Cow::Borrowed("System"),
            author: Cow::Borrowed("System"),
            variants: vec![light_theme, dark_theme],
        }
    }

    pub fn to_theme_family(&self) -> ThemeFamily<'a> {
        ThemeFamily {
            name: Cow::Borrowed(&self.name),
            author: Cow::Borrowed(&self.author),
            variants: self.variants.clone(),
        }
    }
}

impl<'a> ThemeRegistry<'a> {
    pub fn add_system_theme_family(&mut self, system_theme_family: SystemThemeFamily<'a>) {
        self.theme_families.push(system_theme_family.to_theme_family());
    }
}
```