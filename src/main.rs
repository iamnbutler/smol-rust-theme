```rust
use std::collections::BTreeMap;
use crate::ui_color::UIColor;
use crate::color_scale::ColorScale;
use crate::color_scale_set::ColorScaleSet;
use crate::ui_colors::UIColors;
use crate::theme::Theme;

fn main() {
    let mut color_scale_sets: BTreeMap<String, ColorScaleSet> = BTreeMap::new();
    let mut ui_colors: BTreeMap<String, UIColor> = BTreeMap::new();

    // Initialize default ColorScaleSets
    color_scale_sets.insert("gray".to_string(), ColorScaleSet::default());
    color_scale_sets.insert("red".to_string(), ColorScaleSet::default());
    color_scale_sets.insert("green".to_string(), ColorScaleSet::default());
    color_scale_sets.insert("blue".to_string(), ColorScaleSet::default());
    color_scale_sets.insert("yellow".to_string(), ColorScaleSet::default());

    // Initialize UIColors
    ui_colors.insert("filled-element-background".to_string(), UIColor::new("filled-element-background", ColorScale::default(), "Used for the background of filled elements, like buttons and checkboxes."));

    // Create a new theme
    let theme = Theme::new(ui_colors, color_scale_sets);

    // Access a UIColor
    let ui_color = theme.get_ui_color("filled-element-background").unwrap();
    println!("UIColor: {:?}", ui_color);

    // Access a ColorScaleSet
    let color_scale_set = theme.get_color_scale_set("gray").unwrap();
    println!("ColorScaleSet: {:?}", color_scale_set);
}
```