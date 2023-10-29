use std::borrow::Cow;
use crate::utils::hsla::Hsla;
use crate::utils::color_scale::ColorScale;

#[derive(Debug, Clone)]
pub enum ColorValue {
    Static(Hsla),
    Scale(usize),
}

#[derive(Debug, Clone)]
pub struct UIColor<'a> {
    name: Cow<'a, str>,
    value: ColorValue,
    description: Cow<'a, str>,
}

impl<'a> UIColor<'a> {
    pub fn new_static(name: &'a str, value: Hsla, description: &'a str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            value: ColorValue::Static(value),
            description: Cow::Borrowed(description),
        }
    }

    pub fn new_scale(name: &'a str, index: usize, description: &'a str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            value: ColorValue::Scale(index),
            description: Cow::Borrowed(description),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &ColorValue {
        &self.value
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
