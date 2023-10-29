/// Documentation for Theme
///
/// The Theme struct represents a single theme in the system. It has a name, appearance, and a method to map colors to UI elements.
/// It can also have optional overrides. Macros are used to facilitate theme color listing.
///
/// # Examples
///
/// ```
/// let light_theme = Theme::new("Light", Appearance::Light, |ui_color| {
///     match ui_color {
///         UIColor::Background => Hsla::new(0.0, 0.0, 1.0, 1.0),
///         UIColor::Text => Hsla::new(0.0, 0.0, 0.0, 1.0),
///         // ... other UI colors
///     }
/// });
/// ```
///
/// # Note
///
/// UIColor can only be edited via a theme. The theme codebase is designed to be uncomplicated yet thread safe.
/// The system involves both static and dynamic data. The static data includes things like brand color which should be in Hsla.
/// The dynamic data includes things like user-defined themes.
///
/// # See Also
///
/// - `UIColor`: The UI elements that the theme maps colors to.
/// - `Hsla`: The color values used by the theme.
/// - `Appearance`: The appearance (light or dark) of the theme.
/// - `ThemeFamily`: A collection of related themes.
/// - `ThemeRegistry`: The registry where all themes are stored and managed.
///
/// # Safety
///
/// This struct is thread-safe. It does not return any backticks in its code.