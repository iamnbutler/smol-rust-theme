Shared Dependencies:

1. Structs: Hsla, FooStruct, ColorScale, ColorScaleSet, Theme, ThemeFamily, SystemThemeFamily.
2. Functions: hsla.
3. Macros: To be defined in macros.rs for lessening boilerplate code.
4. Data Schemas: UIColors (name, value, description), Hsla units (h, s, l, a), ColorScale (12 Hsla units), ColorScaleSet (4 ColorScale and a name), Theme (name, appearance, method to map colors to UI elements, optionally, overrides), ThemeFamily (name, author, optionally, scales and overrides, plus a vector of Themes), SystemThemeFamily (default light and dark themes).
5. Variables: 'value' in FooStruct, 'h', 's', 'l', 'a' in Hsla.
6. Message Names: Not specified in the prompt.
7. File Paths: "themes/default.toml", "themes/user.toml".
8. Modules: theme_system.
9. Tests: To be defined in tests.rs for major functions.
10. Documentation: To be defined in docs.rs for major concepts and functions.
11. Theme Files: default.toml, user.toml in the 'themes' folder.
12. Theme Elements: App background, Subtle background, UI element, Hovered UI, Active / Selected UI, Subtle borders, UI element border, Hovered UI border, Solid backgrounds, Hovered solid backgrounds, Low-contrast text, High-contrast text.
13. Theme Types: light, dark, light_alpha, dark_alpha.
14. Theme Colors: gray, red, green, blue, yellow.