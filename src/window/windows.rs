use super::*;
use crate::prelude::*;

pub struct WindowsTopBar;

impl WindowsTopBar {
    fn top_resize(window: &mut Window, _: &mut Context<WindowTopBar>) -> impl IntoElement {
        div()
            .h(px(1.))
            .w_full()
            .when(window.is_window_active(), |this| {
                this.bg(gpui::rgb(0xfb94bc))
            })
            .when(!window.is_window_active(), |this| {
                this.bg(gpui::rgb(0x3d3437))
            })
            .cursor_n_resize()
            .on_mouse_down(MouseButton::Left, |_, window, _| {
                window.start_window_resize(ResizeEdge::Top);
            })
    }

    fn close(_: &mut Window, _: &mut Context<WindowTopBar>) -> impl IntoElement {
        h_flex()
            .id("windows-top-bar-close-btn")
            .justify_center()
            .items_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .text_size(px(10.0))
            .hover(|style| {
                style.bg(gpui::Rgba {
                    r: 232.0 / 255.0,
                    g: 17.0 / 255.0,
                    b: 32.0 / 255.0,
                    a: 1.0,
                })
            })
            .window_control_area(WindowControlArea::Close)
            .child("\u{e8bb}")
    }

    fn max(window: &mut Window, cx: &mut Context<WindowTopBar>) -> impl IntoElement {
        h_flex()
            .id("windows-top-bar-max-btn")
            .justify_center()
            .items_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .text_size(px(10.0))
            .hover(|style| style.bg(cx.theme().selection))
            .window_control_area(WindowControlArea::Max)
            .map(|this| {
                if window.is_maximized() {
                    this.child("\u{e923}")
                } else {
                    this.child("\u{e922}")
                }
            })
    }

    fn min(_: &mut Window, cx: &mut Context<WindowTopBar>) -> impl IntoElement {
        h_flex()
            .id("windows-top-bar-min-btn")
            .justify_center()
            .items_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .text_size(px(10.0))
            .hover(|style| style.bg(cx.theme().selection))
            .window_control_area(WindowControlArea::Min)
            .child("\u{e921}")
    }

    fn controls(window: &mut Window, cx: &mut Context<WindowTopBar>) -> impl IntoElement {
        h_flex()
            .id("windows-window-controls")
            .font_family("Segoe MDL2 Assets")
            .text_color(cx.theme().text)
            .justify_center()
            .content_stretch()
            .max_h(px(32.0))
            .min_h(px(32.0))
            .child(Self::min(window, cx))
            .child(Self::max(window, cx))
            .child(Self::close(window, cx))
    }
}

impl WindowTopBarImpl for WindowsTopBar {
    fn render(&self, window: &mut Window, cx: &mut Context<WindowTopBar>) -> AnyElement {
        v_flex()
            .when(!window.is_maximized() && !window.is_fullscreen(), |this| {
                this.child(Self::top_resize(window, cx))
            })
            .child(
                h_flex()
                    .w_full()
                    .content_stretch()
                    .h_8()
                    .window_control_area(WindowControlArea::Drag)
                    .justify_between()
                    .items_center()
                    .text_color(cx.theme().text)
                    .child(div())
                    .child(Self::controls(window, cx)),
            )
            .into_any()
    }
}
