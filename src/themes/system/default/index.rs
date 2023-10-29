use crate::utils::theme::Theme;
use crate::utils::color_scale_set::ColorScaleSet;

pub struct ThemeFamily {
    name: String,
    author: Option<String>,
    scales: Option<ColorScaleSet>,
    overrides: Option<Vec<String>>,
    themes: Vec<Theme>,
}

impl ThemeFamily {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            author: None,
            scales: None,
            overrides: None,
            themes: vec![],
        }
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    pub fn with_scales(mut self, scales: ColorScaleSet) -> Self {
        self.scales = Some(scales);
        self
    }

    pub fn with_overrides(mut self, overrides: Vec<String>) -> Self {
        self.overrides = Some(overrides);
        self
    }

    pub fn add_theme(mut self, theme: Theme) -> Self {
        self.themes.push(theme);
        self
    }
}
