//! Composable glass surfaces using GPUI's backdrop filter.
use gpui_kit::{Div, Hsla, Pixels, Styled, div, hsla, px};

/// Preset glass tint.
#[derive(Clone, Copy, Debug, Default)]
pub enum GlassTint {
    /// A dark translucent surface.
    #[default]
    Dark,
    /// A light translucent surface.
    Light,
    /// An application-supplied color.
    Custom(Hsla),
}

/// Configures a rounded, translucent panel.
///
/// The filter blurs content already painted behind the panel **inside the
/// window**. Platform compositor blur of the desktop is separate. Set
/// `blur` to zero for a low-cost translucent fallback.
#[derive(Clone, Copy, Debug)]
pub struct Glass {
    /// Surface tint.
    pub tint: GlassTint,
    /// Backdrop blur radius in logical pixels.
    pub blur: Pixels,
    /// Corner radius.
    pub radius: Pixels,
}

impl Default for Glass {
    fn default() -> Self {
        Self {
            tint: GlassTint::Dark,
            blur: px(16.),
            radius: px(16.),
        }
    }
}

impl Glass {
    /// Dark glass preset.
    pub fn dark() -> Self {
        Self::default()
    }
    /// Light glass preset.
    pub fn light() -> Self {
        Self {
            tint: GlassTint::Light,
            ..Self::default()
        }
    }
    /// Set the blur radius.
    pub fn blur(mut self, radius: Pixels) -> Self {
        self.blur = radius;
        self
    }
    /// Set the corner radius.
    pub fn radius(mut self, radius: Pixels) -> Self {
        self.radius = radius;
        self
    }
    /// Set a custom translucent tint.
    pub fn tint(mut self, tint: Hsla) -> Self {
        self.tint = GlassTint::Custom(tint);
        self
    }
    /// Build an ordinary GPUI `Div` so callers can add layout and children.
    pub fn build(self) -> Div {
        let (tint, border) = match self.tint {
            GlassTint::Dark => (hsla(0.62, 0.18, 0.12, 0.72), hsla(0., 0., 1., 0.18)),
            GlassTint::Light => (hsla(0., 0., 1., 0.68), hsla(0., 0., 1., 0.7)),
            GlassTint::Custom(tint) => (tint, hsla(0., 0., 1., 0.2)),
        };
        div()
            .rounded(self.radius)
            .border_1()
            .border_color(border)
            .bg(tint)
            .backdrop_blur(self.blur)
            .shadow_lg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::Filter;

    #[test]
    fn glass_configures_a_real_backdrop_filter() {
        let mut panel = Glass::light().blur(px(12.)).build();
        let filters = panel.style().backdrop_filter.as_ref().unwrap();
        assert!(matches!(filters.as_slice(), [Filter::Blur(radius)] if *radius == px(12.)));
        assert!(panel.style().background.is_some());
    }

    #[test]
    fn zero_blur_is_an_explicit_fallback() {
        let mut panel = Glass::dark().blur(px(0.)).build();
        let filters = panel.style().backdrop_filter.as_ref().unwrap();
        assert!(matches!(filters.as_slice(), [Filter::Blur(radius)] if *radius == px(0.)));
    }
}
