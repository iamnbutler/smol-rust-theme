use super::super::utils::theme_registry::ThemeRegistry;
use super::super::utils::theme::Theme;
use super::super::utils::theme_family::ThemeFamily;
use super::super::utils::color_scale_set::ColorScaleSet;
use super::super::utils::color_scale::ColorScale;
use super::super::utils::hsla::Hsla;
use super::super::utils::ui_color::UIColor;

#[test]
fn test_theme_registry() {
    let mut theme_registry = ThemeRegistry::new();

    let hsla = Hsla::new(0.5, 0.5, 0.5, 0.5);
    let color_scale = ColorScale::new([hsla; 12]);
    let color_scale_set = ColorScaleSet::new("test", [color_scale; 4]);
    let ui_color = UIColor::new("test", &color_scale_set, 0, "test");
    let theme = Theme::new("test", vec![ui_color]);
    let theme_family = ThemeFamily::new("test", "test", Some(color_scale_set), vec![theme]);

    theme_registry.add_theme_family(theme_family);

    assert_eq!(theme_registry.get_theme_family("test").unwrap().get_name(), "test");
    assert_eq!(theme_registry.get_theme("test", "test").unwrap().get_name(), "test");
    assert_eq!(theme_registry.get_ui_color("test", "test", "test").unwrap().get_name(), "test");

    theme_registry.remove_theme_family("test");

    assert!(theme_registry.get_theme_family("test").is_none());
    assert!(theme_registry.get_theme("test", "test").is_none());
    assert!(theme_registry.get_ui_color("test", "test", "test").is_none());
}
