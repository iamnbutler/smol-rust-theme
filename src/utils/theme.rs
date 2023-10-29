use std::collections::BTreeMap;
use std::borrow::Cow;
use super::hsla::Hsla;
use super::color_scale::ColorScale;
use super::ui_color::UIColor;

#[derive(Debug, Clone)]
pub struct Theme<'a> {
    name: Cow<'a, str>,
    appearance: Cow<'a, str>,
    color_map: BTreeMap<Cow<'a, str>, UIColor>,
    overrides: Option<BTreeMap<Cow<'a, str>, Hsla>>,
}

impl<'a> Theme<'a> {
    pub fn new(name: &'a str, appearance: &'a str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            appearance: Cow::Borrowed(appearance),
            color_map: BTreeMap::new(),
            overrides: None,
        }
    }

    pub fn with_color(mut self, element: &'a str, color: UIColor) -> Self {
        self.color_map.insert(Cow::Borrowed(element), color);
        self
    }

    pub fn with_override(mut self, element: &'a str, color: Hsla) -> Self {
        match &mut self.overrides {
            Some(overrides) => {
                overrides.insert(Cow::Borrowed(element), color);
            }
            None => {
                let mut overrides = BTreeMap::new();
                overrides.insert(Cow::Borrowed(element), color);
                self.overrides = Some(overrides);
            }
        }
        self
    }

    pub fn get_color(&self, element: &str) -> Option<&UIColor> {
        self.color_map.get(element)
    }

    pub fn get_override(&self, element: &str) -> Option<&Hsla> {
        match &self.overrides {
            Some(overrides) => overrides.get(element),
            None => None,
        }
    }
}
