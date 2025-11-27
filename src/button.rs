use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Button {
    base: Stateful<Div>,
    hover_color: Option<Hsla>,
    base_color: Option<Hsla>,
    bordered: bool,
}

impl Button {
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

    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: v_flex().id(id),
            base_color: None,
            hover_color: None,
            bordered: false,
        }
    }

    pub fn base(self, color: impl Into<Hsla>) -> Self {
        let color = color.into();
        Self {
            base_color: Some(color),
            ..self
        }
    }

    pub fn hover(self, color: impl Into<Hsla>) -> Self {
        Self {
            hover_color: Some(color.into()),
            ..self
        }
    }

    pub fn bordered(self) -> Self {
        Self {
            bordered: true,
            ..self
        }
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements)
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let hover_color = self.hover_color.unwrap_or(cx.theme().foreground);
        self.base
            .hover(|style| style.cursor_pointer().bg(hover_color))
            .when_some(self.base_color, |this, bg| this.bg(bg))
            .when(self.bordered, |this| {
                this.border_1().border_color(cx.theme().border).rounded_lg()
            })
    }
}
