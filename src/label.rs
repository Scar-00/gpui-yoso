use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Center,
    Start,
    End,
}

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    text: SharedString,
    border: bool,
    x_align: TextAlignment,
    y_align: TextAlignment,
}

impl Label {
    padding_style_methods!({
        visibility: pub
    });
    margin_style_methods!({
        visibility: pub
    });

    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: h_flex(),
            text: text.into(),
            border: true,
            x_align: TextAlignment::Center,
            y_align: TextAlignment::Center,
        }
    }

    pub fn align(self, alignment: TextAlignment) -> Self {
        Self {
            x_align: alignment,
            y_align: alignment,
            ..self
        }
    }

    pub fn x_align(self, alignment: TextAlignment) -> Self {
        Self {
            x_align: alignment,
            ..self
        }
    }

    pub fn y_align(self, alignment: TextAlignment) -> Self {
        Self {
            y_align: alignment,
            ..self
        }
    }

    pub fn no_border(self) -> Self {
        Self {
            border: false,
            ..self
        }
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            //.text_center()
            //.items_center()
            //.justify_center()
            .map(|this| match self.x_align {
                TextAlignment::Center => this.justify_center(),
                TextAlignment::Start => this.px_2().justify_start(),
                TextAlignment::End => this.px_2().justify_end(),
            })
            .map(|this| match self.y_align {
                TextAlignment::Center => this.items_center(),
                TextAlignment::Start => this.items_start(),
                TextAlignment::End => this.items_end(),
            })
            .when(self.border, |this| {
                this.border_1().border_color(cx.theme().border).rounded_sm()
            })
            .child(self.text)
    }
}
