//! Small animation helpers backed by GPUI's animation engine.
use gpui_kit::{
    Animation, AnimationElement, AnimationExt, ElementId, IntoElement, Pixels, Styled, px,
};
use std::time::Duration;

/// Animate an element's opacity and vertical offset on entry.
///
/// `id` must be stable across frames and unique among sibling animations.
pub fn fade_in<E: IntoElement + Styled + 'static>(
    element: E,
    id: impl Into<ElementId>,
    duration: Duration,
) -> AnimationElement<E> {
    element.with_animation(
        id,
        Animation::new(duration).with_easing(|t| 1. - (1. - t).powi(3)),
        |element, t| element.opacity(t).mt(px(8. * (1. - t))),
    )
}

/// Animate an element between two explicit sizes and opacity values.
/// This changes layout size rather than applying a GPU transform.
pub fn grow_in<E: IntoElement + Styled + 'static>(
    element: E,
    id: impl Into<ElementId>,
    duration: Duration,
    width: Pixels,
    height: Pixels,
    from_scale: f32,
) -> AnimationElement<E> {
    let from_scale = from_scale.clamp(0., 1.);
    element.with_animation(
        id,
        Animation::new(duration).with_easing(|t| 1. - (1. - t).powi(3)),
        move |element, t| {
            let scale = from_scale + (1. - from_scale) * t;
            element.w(width * scale).h(height * scale).opacity(t)
        },
    )
}

/// GPUI Kit's ready-made fade, slide and size transitions and easing functions.
pub use gpui_kit::base::animation::{
    EffectTransition, ease_in_cubic, ease_in_out_cubic, ease_out_cubic,
};
/// Value-based transitions and motion primitives.
pub use gpui_kit::base::motion;
