use gpui_de::gpui::component::button::{Button, ButtonVariants};
use gpui_de::prelude::*;

struct Counter {
    count: usize,
}

impl Render for Counter {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x101426))
            .child(
                div()
                    .absolute()
                    .top(px(105.))
                    .left(px(170.))
                    .w(px(220.))
                    .h(px(220.))
                    .rounded_full()
                    .bg(rgb(0x5468d4)),
            )
            .child(
                div()
                    .absolute()
                    .top(px(190.))
                    .left(px(300.))
                    .w(px(220.))
                    .h(px(130.))
                    .rounded_lg()
                    .bg(rgb(0xd76b8e)),
            )
            .child(
                Glass::dark()
                    .build()
                    .id("glass-panel")
                    .transitions(|t| t.bg(millis(200)))
                    .hover(|style| style.bg(rgba(0x34435acb)))
                    .p_8()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(format!("Count: {}", self.count))
                    .child(
                        Button::new("increment")
                            .primary()
                            .label("Increment")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.notify();
                            })),
                    ),
            )
    }
}

fn main() {
    gpui_de::run(
        LaunchOptions::new("GPUI-DE counter")
            .dark()
            .size(700., 480.),
        |_, cx| cx.new(|_| Counter { count: 0 }),
    );
}
