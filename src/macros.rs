```rust
#[macro_export]
macro_rules! theme_colors {
    ($($name:expr => $value:expr),* $(,)?) => {
        vec![
            $(
                ThemeColor {
                    name: String::from($name),
                    value: $value,
                    documentation: None,
                },
            )*
        ]
    };
}

#[macro_export]
macro_rules! theme_colors_with_docs {
    ($($name:expr => $value:expr, $doc:expr),* $(,)?) => {
        vec![
            $(
                ThemeColor {
                    name: String::from($name),
                    value: $value,
                    documentation: Some(String::from($doc)),
                },
            )*
        ]
    };
}
```