/// Documentation for UIColor
///
/// UIColor is a struct that represents a color in the theme system. It has a name, value, and description.
/// The value is represented as an Hsla color.
///
/// # Examples
///
/// ```
/// use theme_system::ui_color::UIColor;
/// use theme_system::hsla::Hsla;
///
/// let color = UIColor {
///     name: String::from("brand_color"),
///     value: Hsla::default(),
///     description: String::from("The main brand color"),
/// };
/// ```
///
/// # Notes
///
/// - UIColor can only be edited via a theme.
/// - Respect the UI elements' order; consider using BTreeMap.
/// - Use Hsla for color values; static values like brand color should be in Hsla.
/// - If needed, utilize Cow to avoid cloning strings and handle the dual static and dynamic data.
///
/// # See Also
///
/// - [Hsla](../hsla/struct.Hsla.html)
/// - [Theme](../theme/struct.Theme.html)
pub struct UIColorDocs;