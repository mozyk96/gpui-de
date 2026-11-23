//! Lightweight dockable wrapper for any GPUI view.
use gpui_kit::component::dock::{BasePanel, Panel, PanelEvent};
use gpui_kit::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render, SharedString,
    Window, div, prelude::*,
};

/// Wrap an existing view in a dockable panel without implementing dock traits.
///
/// `name` identifies the panel kind in persisted dock layouts; choose a stable
/// unique name and register a builder before loading persisted layouts.
pub struct ContentPanel<V: Render> {
    name: &'static str,
    title: SharedString,
    view: Entity<V>,
    focus: FocusHandle,
}

impl<V: Render> ContentPanel<V> {
    /// Wrap `view` with a stable panel name and tab title.
    pub fn new(
        name: &'static str,
        title: impl Into<SharedString>,
        view: Entity<V>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            name,
            title: title.into(),
            view,
            focus: cx.focus_handle(),
        }
    }
}

impl<V: Render> EventEmitter<PanelEvent> for ContentPanel<V> {}
impl<V: Render> Focusable for ContentPanel<V> {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
impl<V: Render> BasePanel for ContentPanel<V> {
    fn panel_name(&self) -> &'static str {
        self.name
    }
}
impl<V: Render> Panel for ContentPanel<V> {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.title.clone()
    }
    fn inner_padding(&self, _: &App) -> bool {
        false
    }
}
impl<V: Render> Render for ContentPanel<V> {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.view.clone())
    }
}
