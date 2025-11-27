use crate::prelude::*;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Popup<M: ManagedView> {
    id: ElementId,
    label: SharedString,
    builder: Box<dyn Fn(&mut Window, &mut App) -> Entity<M>>,
}

impl<M: ManagedView> Popup<M> {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        builder: impl Fn(&mut Window, &mut App) -> Entity<M> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            builder: Box::new(builder),
        }
    }
}

impl<M: ManagedView> RenderOnce for Popup<M> {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        CtxMenu::new(self.id.clone())
            .anchor(Corner::TopLeft)
            .attach(Corner::BottomLeft)
            .trigger_btn(MouseButton::Left)
            .trigger(|_, _, _| Button::new((self.id, "trigger")).child(Label::new(self.label)))
            .menu(self.builder)
    }
}
