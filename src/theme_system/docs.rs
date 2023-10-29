```rust
//! # Theme System Documentation
//!
//! This module provides a theme system for Rust applications.
//! It allows for both static and dynamic data, with the ability to easily add or access UIColors.
//! UIColors can only be edited via a theme, ensuring consistency and control over your application's appearance.
//!
//! ## Hsla
//!
//! Hsla is a struct that represents a color in HSLA format. It has four fields: h (hue), s (saturation), l (lightness), and a (alpha).
//! We use this struct and the associated function `hsla` to create and manipulate colors in the theme system.
//!
//! ## FooStruct
//!
//! FooStruct is a struct that manages static system and dynamic user themes using Cow. It has a single field `value` which is a Cow of Hsla.
//!
//! ## ColorScale
//!
//! A ColorScale represents the color range and comprises 12 Hsla units. Each unit has a specific role in the UI, such as background, UI element, border, etc.
//!
//! ## ColorScaleSet
//!
//! A ColorScaleSet contains 4 ColorScale and a name. It provides a convenient way to group related ColorScales together.
//! The default ColorScaleSet includes the colors: gray, red, green, blue, yellow.
//!
//! ## Theme
//!
//! A Theme has a name, appearance, and a method to map colors to UI elements. It can also have optional overrides.
//!
//! ## ThemeFamily
//!
//! A ThemeFamily represents a theme as a whole. It includes a name, author, optionally, scales and overrides, plus a vector of Themes.
//!
//! ## SystemThemeFamily
//!
//! The SystemThemeFamily is a special ThemeFamily that includes default light and dark themes.
//!
//! ## ThemeRegistry
//!
//! The ThemeRegistry is where all themes are stored and managed. It allows for loading system and user themes, modifying system themes and saving them as new user themes, displaying themes and their details, and listing all light/dark themes in alphabetical order.
//!
//! ## Macros
//!
//! Macros are used to lessen boilerplate code and facilitate theme color listing.
//!
//! ## Utils
//!
//! The utils module provides utility functions for the theme system.
//!
//! ## Tests
//!
//! Tests are written for major functions to ensure the quality and correctness of the theme system.
//!
//! ## Theme Files
//!
//! Themes are stored in the `themes` folder as TOML files. You can import a ThemeFamily from a TOML file.
//!
//! ## Non-Goals
//!
//! This module does not provide any UI. It also only documents non-obvious concepts.
//!
//! ## Note
//!
//! Backticks are avoided in code and replaced with `@#@`.
//!
//! ## Quality over Speed
//!
//! The goal of this module is to provide a high-quality, robust theme system, not to be the fastest.
//!
//! ## Clarity for a LLM
//!
//! The code is written to be clear and understandable for a Last Line Maintainer (LLM).
```
