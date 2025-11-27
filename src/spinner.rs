use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Spinner {
    base: Div,
    id: ElementId,
    value: SharedString,
    on_inc: Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>,
    on_dec: Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>,
    inc_el: AnyElement,
    dec_el: AnyElement,
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl Spinner {
    border_style_methods!({
        visibility: pub
    });
    visibility_style_methods!({
        visibility: pub
    });
    padding_style_methods!({
        visibility: pub
    });
    margin_style_methods!({
        visibility: pub
    });
    box_shadow_style_methods!({
        visibility: pub
    });

    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        on_inc: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
        on_dec: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            base: div(),
            id: id.into(),
            value: value.into(),
            on_inc: Box::new(on_inc),
            on_dec: Box::new(on_dec),
            inc_el: svg()
                .text_color(gpui::white())
                .path("chevron_up")
                .flex_none()
                .size_full()
                .into_any_element(), //"+".into_any_element(),
            dec_el: svg()
                .text_color(gpui::white())
                .path("chevron_down")
                .flex_none()
                .size_full()
                .into_any_element(), //"-".into_any_element(),
        }
    }
}

impl RenderOnce for Spinner {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let size = window.text_style().font_size.to_pixels(window.rem_size()) * 2.0;
        self.base
            .id(self.id.clone())
            .flex()
            .flex_row()
            .text_center()
            .justify_center()
            .items_center()
            .text_color(cx.theme().text)
            .child(
                Button::new((self.id.clone(), "dec"))
                    .base(cx.theme().foreground)
                    .hover(cx.theme().selection)
                    .h_full()
                    .w(size)
                    .on_mouse_down(MouseButton::Left, self.on_dec)
                    .child(self.dec_el),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_center()
                    .size_full()
                    .child(self.value),
            )
            .child(
                Button::new((self.id.clone(), "inc"))
                    .base(cx.theme().foreground)
                    .hover(cx.theme().selection)
                    .h_full()
                    .w(size)
                    .on_mouse_down(MouseButton::Left, self.on_inc)
                    .child(self.inc_el),
            )
    }
}
