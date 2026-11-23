//! Application startup and window configuration.

use gpui_kit::{
    AnyWindowHandle, App, Bounds, Entity, Pixels, Render, SharedString, Size, Window, WindowBounds,
    WindowKind, WindowOptions, point, px, size,
};

/// Which color appearance the application should start in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorMode {
    /// Always use the light theme.
    Light,
    /// Always use the dark theme.
    Dark,
    /// Use the operating system / desktop preference at startup.
    #[default]
    System,
}

/// Configuration for [`run`].
///
/// The defaults open a 1280x800 window titled `GPUI-DE`, centered on the
/// primary display, following the system appearance.
#[derive(Clone, Debug)]
pub struct LaunchOptions {
    /// The initial window title.
    pub title: SharedString,
    /// The initial window size (and the size used to center it).
    pub size: Size<Pixels>,
    /// An optional minimum size for user resizing.
    pub min_size: Option<Size<Pixels>>,
    /// The requested color appearance.
    pub color_mode: ColorMode,
    /// The Linux desktop application id, used for window grouping and the
    /// application menu.
    pub app_id: Option<String>,
    /// Request a transparent, compositor-blurred Linux window background.
    /// Other platforms currently ignore this option. Backdrop filters on
    /// [`crate::Glass`] sample content inside the window, not the desktop.
    pub transparent_window: bool,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            title: "GPUI-DE".into(),
            size: size(px(1280.), px(800.)),
            min_size: Some(size(px(640.), px(420.))),
            color_mode: ColorMode::System,
            app_id: None,
            transparent_window: false,
        }
    }
}

impl LaunchOptions {
    /// Create options with a title and the default 1280x800 size.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    /// Set the initial window size.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = size(px(width), px(height));
        self
    }

    /// Set the minimum allowed window size.
    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.min_size = Some(size(px(width), px(height)));
        self
    }

    /// Remove the minimum window size.
    pub fn no_min_size(mut self) -> Self {
        self.min_size = None;
        self
    }

    /// Set the initial color appearance.
    pub fn color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Use the dark theme.
    pub fn dark(self) -> Self {
        self.color_mode(ColorMode::Dark)
    }

    /// Use the light theme.
    pub fn light(self) -> Self {
        self.color_mode(ColorMode::Light)
    }

    /// Set the Linux application id.
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// Request a transparent window background.
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent_window = transparent;
        self
    }

    /// Build the corresponding GPUI window options.
    ///
    /// This does not initialize GPUI or apply a theme. Use it with
    /// [`gpui_kit::open_window`] when managing your own application lifecycle.
    pub fn window_options(&self, cx: &App) -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None, self.size, cx,
            ))),
            window_min_size: self.min_size,
            kind: WindowKind::Normal,
            app_id: self.app_id.clone(),
            titlebar: Some(gpui_kit::TitlebarOptions {
                title: Some(self.title.clone()),
                ..Default::default()
            }),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            linux_window_background: if self.transparent_window {
                gpui_kit::LinuxWindowBackground::Blurred
            } else {
                gpui_kit::LinuxWindowBackground::Opaque
            },
            ..Default::default()
        }
    }
}

/// Run a single-window GPUI-DE application.
///
/// This initializes GPUI, the component library and bundled assets, applies the
/// requested theme, opens one centered window and activates it. The `build`
/// closure receives the window and app so it can construct your root view.
///
/// ```no_run
/// use gpui_de::prelude::*;
///
/// struct App;
///
/// impl Render for App {
///     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
///         div().size_full().child("Hello")
///     }
/// }
///
/// fn main() {
///     gpui_de::run(LaunchOptions::new("My App").dark(), |_, cx| {
///         cx.new(|_| App)
///     });
/// }
/// ```
pub fn run<V>(
    options: LaunchOptions,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
) where
    V: Render + 'static,
{
    let app = gpui_kit::application();
    #[cfg(feature = "assets")]
    let app = app.with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);
        apply_color_mode(options.color_mode, cx);

        let (window, _view) =
            open_window(&options, cx, build).expect("failed to open the main window");

        activate(window, cx);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
    });
}

/// Open a window in an already initialized application, returning any platform
/// error to the caller. Call `gpui_de::gpui::init(cx)` first.
pub fn open_window<V: Render>(
    options: &LaunchOptions,
    cx: &mut App,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
) -> gpui_kit::Result<(AnyWindowHandle, Entity<V>)> {
    gpui_kit::open_window(options.window_options(cx), cx, build)
}

/// Run a single-window application with default options.
///
/// A shorter spelling of [`run`] for examples and small tools.
pub fn run_simple<V>(
    title: impl Into<SharedString>,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
) where
    V: Render + 'static,
{
    run(LaunchOptions::new(title), build);
}

fn activate(window: AnyWindowHandle, cx: &mut App) {
    let _ = window.update(cx, |_, window, _| window.activate());
}

#[cfg(feature = "component")]
fn apply_color_mode(mode: ColorMode, cx: &mut App) {
    use crate::theme;

    match mode {
        ColorMode::Light => theme::set_light(cx),
        ColorMode::Dark => theme::set_dark(cx),
        ColorMode::System => theme::sync_system(None, cx),
    }
}

#[cfg(not(feature = "component"))]
fn apply_color_mode(_mode: ColorMode, _cx: &mut App) {}

/// A convenience alias for `point(px(x), px(y))`.
pub fn p(x: f32, y: f32) -> gpui_kit::Point<Pixels> {
    point(px(x), px(y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_options_preserve_builder_settings() {
        let options = LaunchOptions::new("Example")
            .size(800., 600.)
            .no_min_size()
            .dark()
            .app_id("org.example.app");
        assert_eq!(options.title.as_ref(), "Example");
        assert_eq!(options.size, size(px(800.), px(600.)));
        assert!(options.min_size.is_none());
        assert_eq!(options.color_mode, ColorMode::Dark);
        assert_eq!(options.app_id.as_deref(), Some("org.example.app"));
    }
}
