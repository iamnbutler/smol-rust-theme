```rust
pub mod ui_color;
pub mod color_scale;
pub mod color_scale_set;
pub mod ui_colors;
pub mod theme;

pub use ui_color::UIColor;
pub use color_scale::{ColorScale, Hsla, hsla};
pub use color_scale_set::ColorScaleSet;
pub use ui_colors::UIColors;
pub use theme::Theme;
```