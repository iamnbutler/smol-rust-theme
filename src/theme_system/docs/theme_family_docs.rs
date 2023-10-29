/// Documentation for ThemeFamily
///
/// A ThemeFamily represents a theme as a whole. It includes a name, author, optionally, scales and overrides, 
/// plus a vector of Themes. A ThemeFamily with defined scales need not have any Themes.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use theme_system::theme_family::ThemeFamily;
///
/// let theme_family = ThemeFamily::new("MyTheme", "AuthorName");
/// ```
///
/// Adding a theme to a ThemeFamily:
///
/// ```
/// use theme_system::theme_family::ThemeFamily;
/// use theme_system::theme::Theme;
///
/// let mut theme_family = ThemeFamily::new("MyTheme", "AuthorName");
/// let theme = Theme::new("LightTheme");
/// theme_family.add_theme(theme);
/// ```
///
/// # Note
///
/// The system-to-user theme conversion is possible upon value modification. This means that a user can modify 
/// the system theme and save it as a fresh user theme. This is done through the ThemeRegistry.
///
/// # See also
///
/// [Theme](../theme/struct.Theme.html), [ThemeRegistry](../theme_registry/struct.ThemeRegistry.html)
///
/// [UIColor](../ui_color/struct.UIColor.html), [ColorScale](../color_scale/struct.ColorScale.html), 
/// [ColorScaleSet](../color_scale_set/struct.ColorScaleSet.html)
///
/// [Hsla](../hsla/struct.Hsla.html)