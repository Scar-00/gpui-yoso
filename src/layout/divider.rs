use crate::prelude::*;
use gpui::{prelude::*, *};

pub enum Layout {
    Horizontal,
    Vertical,
}

#[allow(dead_code)]
#[derive(IntoElement)]
pub struct Divider {
    layout: Layout,
    color: Option<gpui::Hsla>,
    thickness: gpui::Length,
    inset: Option<gpui::Length>,
}

#[allow(dead_code)]
impl Divider {
    pub fn vertical() -> Self {
        Self {
            layout: Layout::Vertical,
            color: None,
            thickness: gpui::Length::Definite(gpui::px(1.0).into()),
            inset: None,
        }
    }

    pub fn horizontal() -> Self {
        Self {
            layout: Layout::Horizontal,
            color: None,
            thickness: gpui::Length::Definite(gpui::px(1.0).into()),
            inset: None,
        }
    }

    pub fn color(mut self, color: impl Into<gpui::Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn thickness(mut self, thickness: impl Into<gpui::Length>) -> Self {
        self.thickness = thickness.into();
        self
    }

    pub fn inset(mut self, inset: impl Into<gpui::Length>) -> Self {
        self.inset = Some(inset.into());
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .map(|this| match self.layout {
                Layout::Vertical => this
                    .w(self.thickness)
                    .h_full()
                    .when_some(self.inset, |this, _| this.my_1p5() /*.my(inset)*/),
                Layout::Horizontal => this
                    .h(self.thickness)
                    .w_full()
                    .when_some(self.inset, |this, _| this.mx_1p5()),
            })
            .bg(self.color.unwrap_or(cx.theme().border))
    }
}
