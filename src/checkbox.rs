use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Checkbox {
    base: Stateful<Div>,
    selected: bool,
    label: Option<AnyElement>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, selected: bool) -> Self {
        Self {
            base: h_flex().id(id),
            selected,
            label: None,
        }
    }

    pub fn label(self, label: impl IntoElement) -> Self {
        Self {
            label: Some(label.into_any_element()),
            ..self
        }
    }
}

impl InteractiveElement for Checkbox {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .items_center()
            .gap_1()
            .when_some(self.label, |this, label| this.child(label))
            .child(
                div()
                    .size_5()
                    .border_1()
                    .border_color(cx.theme().border)
                    .when(self.selected, |this| {
                        this.child(
                            svg()
                                .text_color(gpui::white())
                                .size_full()
                                .path("checkbox-inner"),
                        )
                    }),
            )
    }
}
