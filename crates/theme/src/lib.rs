/*!

This crate provides the default theme resources of OrbTks default theme dark and light.
It provides fonts, icons and colors.

 */

use orbtk_theming::{config::ThemeConfig, prelude::*};

/// provides `constants` to reference colors.
pub mod colors;
/// provides `constants` associated to fonts.
pub mod fonts;
pub mod prelude;
/// provides information processed by the `graphic render` (e.g. glyphs, icons).
pub mod vector_graphics;

/// The dark theme resource file.
pub const DARK_THEME_RON: &str = include_str!("../assets/dark/dark.ron");

/// The default theme colors resource file.
pub const COLORS_RON: &str = include_str!("../assets/common/colors.ron");

/// The common fonts resource file.
pub const FONTS_RON: &str = include_str!("../assets/common/fonts.ron");

/// The light theme resource file.
pub const LIGHT_THEME_RON: &str = include_str!("../assets/light/light.ron");

/// Creates OrbTks default dark theme.
pub fn dark_theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

/// Creates OrbTks default light theme.
pub fn light_theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(LIGHT_THEME_RON)
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}
