```rust
use super::theme::Theme;
use super::color_scale_set::ColorScaleSet;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct SystemThemeFamily {
    pub light_theme: Theme,
    pub dark_theme: Theme,
}

impl SystemThemeFamily {
    pub fn new() -> Self {
        let light_theme = Theme::new("default_light", "light", ColorScaleSet::default(), BTreeMap::new());
        let dark_theme = Theme::new("default_dark", "dark", ColorScaleSet::default(), BTreeMap::new());

        SystemThemeFamily {
            light_theme,
            dark_theme,
        }
    }
}
```