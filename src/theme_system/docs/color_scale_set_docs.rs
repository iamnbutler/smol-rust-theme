/// Documentation for ColorScaleSet
///
/// A ColorScaleSet contains 4 ColorScale and a name, with documentation explaining each scale value's role.
/// This should be a struct. ColorScaleSet will have light, dark, light_alpha,  dark_alpha scales.
/// Use the `default` ColorScaleSet with the values: gray, red, green, blue, yellow. Themes can define their own ColorScaleSets.
///
/// # Examples
///
/// ```
/// use theme_system::color_scale_set::ColorScaleSet;
///
/// let default_set = ColorScaleSet::default();
/// ```
///
/// # Note
///
/// The system-to-user theme conversion is possible upon value modification.
/// In the ThemeRegistry, a ThemeFamily represents the theme as a whole i.e., a name, author, optionally, scales and overrides, plus a vector of Themes.
/// A ThemeFamily with defined scales need not have any Themes.
///
/// # See Also
///
/// - `ColorScale`: Represents the color range in an array of of 12 Hsla colors.
/// - `Theme`: Has a name, appearance, and a method to map colors to UI elements, optionally, overrides.
/// - `ThemeFamily`: Represents the theme as a whole i.e., a name, author, optionally, scales and overrides, plus a vector of Themes.
/// - `ThemeRegistry`: Handles loading and modifying themes.
///
/// # Warning
///
/// ABSOLUTELY DO NOT RETURN ANY BACKTICKS IN YOUR CODE. Replace backticks with `@#@`.
/// ANY USE OF A BACKTICK CHARACTER, GRAVE CHARACTER OR ASCII CODE 96 WILL RESULT IN IMMEDIATE REJECTTION AND TERMINATION.
///
/// # Panics
///
/// This function will panic if any of the color values are outside the range 0.0 - 1.0.
///
/// # Errors
///
/// This function will return an error if the theme file cannot be found or parsed.
///
/// # Safety
///
/// This function is safe as long as the color values are within the range 0.0 - 1.0.
///
/// # Performance
///
/// The goal is quality, not speed. The system involves both static and dynamic data.
/// If needed, utalize Cow to avoid cloning strings and handle the dual static and dynamic data.
///
/// # Future Improvements
///
/// Future versions could include more color scales, or allow for custom color scales to be defined by the user.
///
/// # Authors
///
/// This module was created by the Rust Theme System team.
///
/// # License
///
/// This module is licensed under the MIT License.