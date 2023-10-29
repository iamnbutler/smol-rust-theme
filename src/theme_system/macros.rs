```rust
#[macro_export]
macro_rules! define_colors {
    ($($name:ident : $h:expr, $s:expr, $l:expr, $a:expr),* $(,)?) => {
        $(
            pub const $name: Hsla = hsla($h, $s, $l, $a);
        )*
    };
}

#[macro_export]
macro_rules! define_theme {
    ($name:ident, $appearance:expr, $($element:ident : $color:ident),* $(,)?) => {
        pub struct $name {
            appearance: &'static str,
            $($element: UIColor),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    appearance: $appearance,
                    $($element: UIColor {
                        name: stringify!($element),
                        value: $color,
                        description: stringify!($color),
                    }),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_theme_family {
    ($name:ident, $author:expr, $($theme:ident),* $(,)?) => {
        pub struct $name {
            name: &'static str,
            author: &'static str,
            themes: Vec<Box<dyn Theme>>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    name: stringify!($name),
                    author: $author,
                    themes: vec![$(Box::new($theme::new())),*],
                }
            }
        }
    };
}
```