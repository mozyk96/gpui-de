//! Helpers for a real, resizable GPUI Kit dock workspace.
use gpui_kit::component::dock::{
    DockArea, DockLayout, DockPlacement, DockSkin, Panel, panel_handle,
};
use gpui_kit::{
    App, Context, Entity, IntoElement, Pixels, Render, SharedString, Window, div, prelude::*, px,
};

/// Build a tab group from dockable panels.
pub fn tabs<P: Panel>(panels: impl IntoIterator<Item = Entity<P>>, cx: &App) -> DockLayout {
    panels
        .into_iter()
        .fold(DockLayout::tabs(), |layout, panel| {
            layout.panel_view(panel_handle(panel), cx)
        })
}

/// A skinned dock area with center tabs and optional side/bottom docks.
///
/// The underlying [`DockArea`] remains accessible for persistence, custom
/// layouts, and dynamic panel management.
pub struct Workspace {
    area: Entity<DockArea>,
}

impl Workspace {
    /// Create a workspace with a center layout.
    pub fn new(
        id: impl Into<SharedString>,
        center: DockLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (area, _) = DockSkin::dock_area(id, None, window, cx);
        area.update(cx, |area, cx| area.set_center(center, window, cx));
        Self { area }
    }

    /// Add a dock at an edge with the requested initial width or height.
    pub fn dock(
        self,
        placement: DockPlacement,
        layout: DockLayout,
        extent: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        self.area.update(cx, |area, cx| {
            area.set_dock(placement, layout, window, cx);
            area.set_dock_size(placement, extent, window, cx);
            area.set_dock_collapsible(placement, true, window, cx);
        });
        self
    }

    /// Access the underlying GPUI Kit dock area.
    pub fn area(&self) -> &Entity<DockArea> {
        &self.area
    }
}

impl Render for Workspace {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().min_h(px(0.)).child(self.area.clone())
    }
}
