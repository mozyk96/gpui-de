//! # GPUI Developer Edition
//!
//! `gpui-de` is a small, opinionated layer on top of the
//! [GPUI Community Edition](https://github.com/gpui-ce/gpui-ce) toolkit. It
//! packages application startup, theming, dock layouts, animation helpers and
//! a reusable glass effect so a native desktop app can be started in
//! a few dozen lines.
//!
//! It deliberately stays close to GPUI: every helper returns ordinary GPUI
//! elements, and the full GPUI and GPUI Kit APIs remain reachable.
//!
//! ## Quick start
//!
//! ```no_run
//! use gpui_de::prelude::*;
//!
//! struct Hello;
//!
//! impl Render for Hello {
//!     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
//!         div()
//!             .size_full()
//!             .flex()
//!             .items_center()
//!             .justify_center()
//!             .child("Hello from GPUI-DE")
//!     }
//! }
//!
//! fn main() {
//!     gpui_de::run(
//!         LaunchOptions::new("Hello"),
//!         |_, cx| cx.new(|_| Hello),
//!     );
//! }
//! ```
//!
//! For dock layouts see [`Workspace`] and [`ContentPanel`].

pub use gpui_kit as gpui;
pub use gpui_kit::base;

pub mod animation;
pub mod app;
pub mod effects;
#[cfg(feature = "component")]
pub mod panel;
pub mod prelude;
#[cfg(feature = "component")]
pub mod theme;
#[cfg(feature = "component")]
pub mod workspace;

pub use app::{ColorMode, LaunchOptions, open_window, run, run_simple};
pub use effects::{Glass, GlassTint};

#[cfg(feature = "component")]
pub use panel::ContentPanel;
#[cfg(feature = "component")]
pub use theme::{
    ActiveTheme, Theme, ThemeColor, ThemeConfig, ThemeMode, accent, customize, set_dark, set_light,
    set_mode, toggle,
};
#[cfg(feature = "component")]
pub use workspace::{Workspace, tabs};

/// The version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
