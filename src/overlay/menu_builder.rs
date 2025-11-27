use std::rc::Rc;

use crate::prelude::*;
use gpui::{prelude::*, *};

actions!(
    menu,
    [
        /// Cancels the current menu operation.
        Cancel,
        /// Confirms the selected menu item.
        Confirm,
        /// Performs secondary confirmation action.
        SecondaryConfirm,
        /// Selects the previous item in the menu.
        SelectPrevious,
        /// Selects the next item in the menu.
        SelectNext,
        /// Selects the first item in the menu.
        SelectFirst,
        /// Selects the last item in the menu.
        SelectLast,
        /// Restarts the menu from the beginning.
        Restart,
        EndSlot,
    ]
);

pub enum MenuListItem {
    NonInteractive(Box<dyn Fn(&mut Window, &mut App) -> AnyElement>),
    InteractiveElement {
        render: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
        handler: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    },
}

pub struct MenuBuilder {
    //builder: Option<Rc<dyn Fn(Self, &mut Window, &mut Context<Self>) -> Self>>,
    items: Vec<MenuListItem>,
    focus: FocusHandle,
    delayed: bool,
    keep_open_on_confirm: bool,
    fixed_width: Option<DefiniteLength>,
    interactive_accent_color: Option<Hsla>,
    rounding: AbsoluteLength,
}

impl MenuBuilder {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        builder: impl Fn(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Self {
        let focus = cx.focus_handle();
        builder(
            Self {
                //builder: None,
                items: Vec::new(),
                focus,
                //action_context: None,
                //selected_index: None,
                delayed: false,
                //clicked: false,
                //end_slot_action: None,
                keep_open_on_confirm: false,
                fixed_width: None,
                interactive_accent_color: None,
                rounding: AbsoluteLength::Pixels(px(0.0)),
            },
            window,
            cx,
        )
    }

    pub fn build(
        window: &mut Window,
        cx: &mut App,
        builder: impl Fn(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, builder))
    }

    pub fn non_interactive(
        mut self,
        e: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.items.push(MenuListItem::NonInteractive(Box::new(e)));
        self
    }

    pub fn interactive(
        mut self,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(MenuListItem::InteractiveElement {
            render: Box::new(render),
            handler: Some(Rc::new(handler)),
        });
        self
    }

    pub fn rounded(mut self, rounding: impl Into<AbsoluteLength>) -> Self {
        self.rounding = rounding.into();
        self
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if self.keep_open_on_confirm {
            todo!();
        } else {
            cx.emit(DismissEvent);
        }
    }
}

impl Focusable for MenuBuilder {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl EventEmitter<DismissEvent> for MenuBuilder {}

impl Render for MenuBuilder {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .occlude()
            .bg(cx.theme().foreground)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().border)
            .flex_shrink_0()
            .child(
                v_flex()
                    .gap(px(4.0))
                    .p_1()
                    .id("context-menu-builder")
                    .max_h(Length::from(window.viewport_size().height * 0.75))
                    .flex_shrink_0()
                    .when_some(self.fixed_width, |this, width| {
                        this.w(width).overflow_x_hidden()
                    })
                    .when(self.fixed_width.is_none(), |this| {
                        this.min_w(px(200.0)).flex_1()
                    })
                    .overflow_y_scroll()
                    .track_focus(&self.focus_handle(cx))
                    .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                        this.cancel(&Cancel, window, cx);
                    }))
                    .key_context("menu")
                    .on_action(cx.listener(Self::cancel))
                    .on_action(cx.listener(Self::confirm))
                    .when(!self.delayed, |this| {
                        let interactive_accent_color = self
                            .interactive_accent_color
                            .unwrap_or(cx.theme().selection);
                        let rounding = self.rounding;
                        this.children(self.items.iter().enumerate().map(|(i, item)| match item {
                            MenuListItem::NonInteractive(render) => {
                                render(window, cx).into_any_element()
                            }
                            MenuListItem::InteractiveElement { render, handler } => {
                                let menu = cx.entity().downgrade();
                                let handler = handler.clone();
                                Button::new(("ctx-btn", i))
                                    .rounded(rounding)
                                    .child(render(window, cx))
                                    .hover(interactive_accent_color)
                                    .when_some(handler, |this, handler| {
                                        this.on_mouse_down(
                                            MouseButton::Left,
                                            move |_, window, cx| {
                                                handler(window, cx);
                                                _ = menu.update(cx, |_, cx| {
                                                    cx.emit(DismissEvent);
                                                });
                                            },
                                        )
                                    })
                                    .into_any_element()
                            }
                        }))
                    }),
            )
    }
}
