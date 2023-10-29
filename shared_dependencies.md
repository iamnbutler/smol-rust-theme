1. `Hsla`: A color value used across the theme system. It is shared in `theme.rs`, `theme_color.rs`, and `system_theme_family.rs`.

2. `Cow`: A type used to allow static system themes and dynamic user themes. It is shared in `theme.rs` and `system_theme_family.rs`.

3. `ThemeRegistry`: A struct that contains `ThemeFamily`s. It is shared in `main.rs` and `theme_registry.rs`.

4. `ThemeFamily`: A struct that has a name, an author, and a vec of `variant`, which is a vec of `Theme`. It is shared in `theme_registry.rs`, `theme_family.rs`, and `system_theme_family.rs`.

5. `Theme`: A struct that has a name, appearance (light/dark), and a vec of `ThemeColor`. It is shared in `theme_family.rs`, `theme.rs`, and `system_theme_family.rs`.

6. `ThemeColor`: A struct that has a name, a value, and an optional documentation string. It is shared in `theme.rs` and `theme_color.rs`.

7. `SystemThemeFamily`: A struct that has a default light and dark theme. It is shared in `main.rs` and `system_theme_family.rs`.

8. `themes/sample_user_theme_family.toml` and `themes/edited_default_theme.toml`: TOML files used for loading and saving themes. They are shared in `main.rs`.

9. Macros: Used to build lists of theme colors. They are shared in `theme_color.rs` and `macros.rs`.

10. Function names: `load`, `edit`, `save`, `print`, `list_all_light_themes`, `list_all_dark_themes`. These are shared across `main.rs`, `theme_registry.rs`, and `system_theme_family.rs`.