use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    focus: FocusHandle,
    children: Vec<AnyElement>,
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>, focus: FocusHandle) -> Self {
        Self {
            id: id.into(),
            focus,
            children: Vec::new(),
        }
    }
}

impl RenderOnce for TabBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .track_focus(&self.focus)
            .bg(cx.theme().foreground)
            .w_full()
            .h_9()
            .p_0p5()
            .gap_0p5()
            .items_center()
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct Tab {
    base: Button,
    inner: AnyElement,
    selected: bool,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>, inner: impl IntoElement) -> Self {
        Self {
            base: Button::new(id),
            inner: inner.into_any_element(),
            selected: false,
        }
    }

    pub fn selected(self, selected: bool) -> Self {
        Self { selected, ..self }
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Tab {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .px_2()
            .h_full()
            .bg(cx.theme().background)
            .when(self.selected, |this| this.bg(cx.theme().selection))
            .hover(cx.theme().selection)
            .child(self.inner)
    }
}
