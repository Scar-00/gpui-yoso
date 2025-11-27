use std::borrow::Borrow;

use crate::prelude::*;
use gpui::{prelude::*, *};

pub struct Tooltip {
    text: SharedString,
}

impl Tooltip {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

/*impl RenderOnce for Tooltip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        deferred(
            anchored()
                .anchor(Corner::TopRight)
                .snap_to_window_with_margin(px(8.0))
                .offset(point(px(2.0), px(8.0)))
                .child(
                    div()
                        .bg(cx.theme().muted_background)
                        //.border_2()
                        //.border_color(cx.theme().border)
                        .rounded_lg()
                        .child(v_flex().py_1().px_2().child(self.text)),
                ),
        )
        .with_priority(1)
    }
}*/

pub fn tooltip_container<C>(cx: &mut C, f: impl FnOnce(Div, &mut C) -> Div) -> impl IntoElement
where
    C: AppContext + Borrow<App>,
{
    let app = (*cx).borrow();
    let theme = Theme::global(app);

    // padding to avoid tooltip appearing right below the mouse cursor
    div().pl_2().pt_2p5().child(
        v_flex()
            .bg(theme.secondary)
            .rounded_lg()
            .border_1()
            .border_color(theme.border)
            //.shadow(index.shadow(cx))
            //.font(ui_font)
            //.text_ui(app)
            .text_color(theme.text)
            .py_1()
            .px_2()
            .map(|el| f(el, cx)),
    )
}

impl Render for Tooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |this, _| this.child(self.text.clone()))
    }
}
