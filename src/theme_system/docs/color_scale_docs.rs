/// Documentation for ColorScale
///
/// A ColorScale represents the color range in an array of 12 Hsla colors. 
/// Each unit in the array has a specific role in the theme system. 
/// The roles are as follows:
///
/// 1. App background
/// 2. Subtle background
/// 3. UI element
/// 4. Hovered UI
/// 5. Active / Selected UI
/// 6. Subtle borders
/// 7. UI element border
/// 8. Hovered UI border
/// 9. Solid backgrounds
/// 10. Hovered solid backgrounds
/// 11. Low-contrast text
/// 12. High-contrast text
///
/// These roles are only used for reference when defining the UIColors. 
/// The actual mapping of colors to UI elements is done in the Theme struct.
///
/// A ColorScale is part of a ColorScaleSet, which contains 4 ColorScales and a name. 
/// The ColorScaleSet will have light, dark, light_alpha, and dark_alpha scales. 
/// The default ColorScaleSet has the values: gray, red, green, blue, yellow. 
/// Themes can define their own ColorScaleSets.
///
/// The ColorScale is used in the system-to-user theme conversion, which is possible upon value modification. 
/// The ThemeRegistry uses the ColorScale in the representation of a ThemeFamily, 
/// which represents the theme as a whole i.e., a name, author, optionally, scales and overrides, 
/// plus a vector of Themes. A ThemeFamily with defined scales need not have any Themes.
///
/// The Theme struct has a name, appearance, and a method to map colors to UI elements, 
/// optionally, overrides. Macros can facilitate theme color listing.
///
/// The ColorScale is stored in the `themes` folder as part of the ThemeFamily toml file.
///
/// The ColorScale is part of the SystemThemeFamily, which has default light and dark themes.
///
/// The ColorScale is designed to be uncomplicated yet thread safe, and to allow for easy addition or access of UIColors. 
/// UIColors can only be edited via a theme.
///
/// The ColorScale is part of a Rust theme system that involves both static and dynamic data. 
/// The goal of the system is quality, not speed. The system does not include any UI. 
/// The system commits work often, not in a single commit. 
/// Tests and documentation are colocated with the appropriate code. 
/// The system does not use backticks in code. 
/// The system maintains clarity for a LLM. 
/// The system does not return any backticks in code.