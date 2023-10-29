```rust
mod hsla_tests;
mod color_scale_tests;
mod color_scale_set_tests;
mod ui_color_tests;
mod theme_tests;
mod theme_family_tests;
mod theme_registry_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_all_tests() {
        hsla_tests::run_tests();
        color_scale_tests::run_tests();
        color_scale_set_tests::run_tests();
        ui_color_tests::run_tests();
        theme_tests::run_tests();
        theme_family_tests::run_tests();
        theme_registry_tests::run_tests();
    }
}
```