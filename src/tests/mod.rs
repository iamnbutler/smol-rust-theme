pub mod hsla_tests;
pub mod color_scale_tests;
pub mod color_scale_set_tests;
pub mod ui_color_tests;
pub mod theme_tests;
pub mod theme_family_tests;
pub mod theme_registry_tests;

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
