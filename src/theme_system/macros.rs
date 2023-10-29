```rust
#[macro_export]
macro_rules! define_theme {
    ($name:ident, $appearance:expr, { $($element:ident : $color:expr),* }) => {
        pub struct $name;

        impl Theme for $name {
            fn name() -> &'static str {
                stringify!($name)
            }

            fn appearance() -> Appearance {
                $appearance
            }

            fn color_for_element(&self, element: &Element) -> Hsla {
                match element {
                    $(Element::$element => $color,)*
                    _ => panic!("Undefined color for element {:?}", element),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_theme_family {
    ($name:ident, $author:expr, $scales:expr, $overrides:expr, { $($theme:ident),* }) => {
        pub struct $name;

        impl ThemeFamily for $name {
            fn name() -> &'static str {
                stringify!($name)
            }

            fn author() -> &'static str {
                $author
            }

            fn scales() -> Option<&'static ColorScaleSet> {
                $scales
            }

            fn overrides() -> Option<&'static Overrides> {
                $overrides
            }

            fn themes() -> Vec<Box<dyn Theme>> {
                vec![
                    $(Box::new($theme),)*
                ]
            }
        }
    };
}
```