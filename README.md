# GPUI Developer Edition

GPUI-DE is a small Rust toolkit for native desktop applications. It builds
on GPUI Community Edition and its compatible GPUI Kit, retaining access to
their full APIs while providing straightforward startup, dock and glass
helpers.

## Installation

```toml
[dependencies]
gpui-de = "0.1"
```

For this checkout, use `gpui-de = { path = "../gpui-de" }` instead. Rust 1.95
or newer is required. Linux builds require the usual Wayland/X11, fontconfig
and graphics development libraries.

```rust,no_run
use gpui_de::prelude::*;

struct Hello;
impl Render for Hello {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().items_center().justify_center()
            .child("Hello from GPUI-DE")
    }
}
fn main() {
    gpui_de::run(LaunchOptions::new("Hello").dark(), |_, cx| cx.new(|_| Hello));
}
```

Run the examples from the repository:

```sh
cargo run --example minimal
```

## API

- `run` and `LaunchOptions` initialize GPUI, assets, theme, and a window.
  For custom lifecycle or error handling, call `gpui_de::gpui::init` and use
  `open_window`, which returns GPUI's `Result`.
- `gpui_de::gpui` re-exports GPUI Kit, including its `component` and `base`
  namespaces. Use its controls, editor, dock serialization and lower-level
  GPUI functionality directly.
- `Workspace`, `tabs` and `ContentPanel` simplify dock integration.
- `theme` provides light/dark switching and `Theme::update`-based custom colors.
- `Glass` creates a rounded translucent surface using GPUI's backdrop
  filter. `animation` exposes entry effects and existing motion primitives.

The default features are `component` and `assets`. Disable them with
`default-features = false` for the base-only layer. `tree-sitter` enables the
Kit's optional language grammars. `test-support` enables upstream test APIs.
Theme mode follows the system only at startup; call `theme::sync_system` to
apply a later appearance change. Without `component`, theme options are inert.

### Effects and limitations

Backdrop blur samples previously drawn content **inside the window**, not
arbitrary windows behind it. A transparent Linux window requests compositor
blur, which depends on the compositor. Set `Glass::blur(px(0.))` for a
translucent fallback. Liquid Glass is an appearance inspired by frosted glass,
not a physical refraction effect. Custom WGSL and general GPU transforms are
deferred; GPUI Fusion's renderer is a different GPUI implementation and is
not linked. The size-based `grow_in` animation affects layout.

## License and attribution

Apache-2.0; see [LICENSE-APACHE](LICENSE-APACHE) and [NOTICE](NOTICE). GPUI-DE
depends on the Apache-2.0 GPUI-CE and GPUI-CE Kit ecosystem. Their upstream
licenses and bundled asset licenses apply to those dependencies.
