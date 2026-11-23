//! Theming helpers built on GPUI Kit's semantic theme.
//!
//! The complete [`Theme`] is still available; these functions are short forms
//! for the operations applications perform at startup and from a settings
//! menu.

pub use gpui_kit::component::{ActiveTheme, Theme};
use gpui_kit::{App, ColorExt, Hsla, Pixels, Window};

pub use gpui_kit::component::{
    ThemeColor, ThemeConfig, ThemeMode, ThemeRegistry, ThemeStyled, ThemeTokens,
};

/// Switch to the light theme.
pub fn set_light(cx: &mut App) {
    Theme::change(ThemeMode::Light, None, cx);
}

/// Switch to the dark theme.
pub fn set_dark(cx: &mut App) {
    Theme::change(ThemeMode::Dark, None, cx);
}

/// Set a specific theme mode.
pub fn set_mode(mode: impl Into<ThemeMode>, window: Option<&mut Window>, cx: &mut App) {
    Theme::change(mode, window, cx);
}

/// Apply the current operating system appearance once. Call this again when
/// the platform appearance changes if your application needs live updates.
pub fn sync_system(window: Option<&mut Window>, cx: &mut App) {
    Theme::sync_system_appearance(window, cx);
}

/// Toggle between light and dark.
pub fn toggle(window: Option<&mut Window>, cx: &mut App) -> ThemeMode {
    let next = if Theme::global(cx).is_dark() {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    };
    Theme::change(next, window, cx);
    next
}

/// Edit the global theme and refresh every window.
///
/// This is the safe write path: it keeps the color tokens, the component
/// tokens and the base theme in step.
///
/// ```no_run
/// use gpui_de::prelude::*;
///
/// fn brand(cx: &mut App, blue: Hsla) {
///     gpui_de::customize(cx, |theme| {
///         theme.colors.primary = blue;
///         theme.radius = px(8.);
///     });
/// }
/// ```
pub fn customize(cx: &mut App, edit: impl FnOnce(&mut Theme)) {
    Theme::update(cx, edit);
}

/// Set the primary/accent color used by interactive controls.
pub fn accent(cx: &mut App, color: Hsla) {
    Theme::update(cx, |theme| {
        theme.colors.primary = color;
        theme.colors.primary_hover = color;
        theme.colors.primary_active = color;
        theme.colors.button_primary = color;
        theme.colors.button_primary_hover = color;
        theme.colors.button_primary_active = color;
        theme.colors.accent = color.opacity(0.18);
        theme.colors.accent_foreground = color;
        theme.colors.ring = color;
    });
}

/// Set the global corner radius, in pixels.
pub fn radius(cx: &mut App, radius: Pixels) {
    Theme::update(cx, |theme| {
        theme.radius = radius;
        theme.radius_lg = radius * 1.5;
    });
}

/// Set the UI and monospace font families.
pub fn font_family(
    cx: &mut App,
    ui: impl Into<gpui_kit::SharedString>,
    mono: impl Into<gpui_kit::SharedString>,
) {
    let ui = ui.into();
    let mono = mono.into();
    Theme::update(cx, |theme| {
        theme.font_family = ui;
        theme.mono_font_family = mono;
    });
}

/// The current theme's background color.
pub fn background(cx: &App) -> Hsla {
    Theme::global(cx).background
}

/// The current theme's primary color.
pub fn primary(cx: &App) -> Hsla {
    Theme::global(cx).primary
}

/// A muted foreground color suitable for secondary labels.
pub fn muted(cx: &App) -> Hsla {
    Theme::global(cx).muted_foreground
}

/// A translucent foreground overlay for glass surfaces.
pub fn glass_tint(cx: &App) -> Hsla {
    let theme = Theme::global(cx);
    if theme.is_dark() {
        theme.background.opacity(0.55)
    } else {
        theme.background.opacity(0.62)
    }
}

/// A hairline color for glass borders.
pub fn glass_border(cx: &App) -> Hsla {
    let theme = Theme::global(cx);
    if theme.is_dark() {
        gpui_kit::hsla(0., 0., 1., 0.14)
    } else {
        gpui_kit::hsla(0., 0., 0., 0.10)
    }
}
