use crate::prelude::*;
use gpui::{prelude::*, *};
use std::{cell::RefCell, panic::Location, rc::Rc};

pub struct CtxMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<Box<dyn FnOnce(bool, &mut Window, &mut App) -> AnyElement + 'static>>,
    menu_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> Entity<M> + 'static>>,
    anchor: Option<Corner>,
    attach: Option<Corner>,
    trigger_button: MouseButton,
}

impl<M: ManagedView> CtxMenu<M> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            child_builder: None,
            menu_builder: None,
            anchor: None,
            attach: None,
            trigger_button: MouseButton::Right,
        }
    }

    pub fn menu(self, builder: impl Fn(&mut Window, &mut App) -> Entity<M> + 'static) -> Self {
        Self {
            menu_builder: Some(Rc::new(builder)),
            ..self
        }
    }

    pub fn trigger_btn(self, btn: MouseButton) -> Self {
        Self {
            trigger_button: btn,
            ..self
        }
    }

    pub fn trigger<F, E>(self, trigger: F) -> Self
    where
        F: FnOnce(bool, &mut Window, &mut App) -> E + 'static,
        E: IntoElement + 'static,
    {
        Self {
            child_builder: Some(Box::new(move |is_active, window, cx| {
                trigger(is_active, window, cx).into_any_element()
            })),
            ..self
        }
    }

    pub fn anchor(self, anchor: Corner) -> Self {
        Self {
            anchor: Some(anchor),
            ..self
        }
    }

    pub fn attach(self, corner: Corner) -> Self {
        Self {
            attach: Some(corner),
            ..self
        }
    }

    fn with_element_state<R>(
        &mut self,
        global_id: &GlobalElementId,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(&mut Self, &mut MenuHandleElementState<M>, &mut Window, &mut App) -> R,
    ) -> R {
        window.with_optional_element_state::<MenuHandleElementState<M>, _>(
            Some(global_id),
            |element_state, window| {
                let mut element_state = element_state.unwrap().unwrap_or_default();
                let result = f(self, &mut element_state, window, cx);
                (result, Some(element_state))
            },
        )
    }
}

struct MenuHandleElementState<M> {
    menu: Rc<RefCell<Option<Entity<M>>>>,
    pos: Rc<RefCell<Point<Pixels>>>,
}

impl<M> Clone for MenuHandleElementState<M> {
    fn clone(&self) -> Self {
        Self {
            menu: Rc::clone(&self.menu),
            pos: Rc::clone(&self.pos),
        }
    }
}

impl<M> Default for MenuHandleElementState<M> {
    fn default() -> Self {
        Self {
            menu: Default::default(),
            pos: Default::default(),
        }
    }
}

pub struct RequestLayoutState {
    child_layout_id: Option<LayoutId>,
    child_element: Option<AnyElement>,
    menu_element: Option<AnyElement>,
}

pub struct PrepaintState {
    hitbox: Hitbox,
    child_bounds: Option<Bounds<Pixels>>,
}

impl<M: ManagedView> Element for CtxMenu<M> {
    type RequestLayoutState = RequestLayoutState;
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.with_element_state(id.unwrap(), window, cx, |this, state, window, cx| {
            let mut menu_layout_id = None;

            let menu_element = state.menu.borrow_mut().as_mut().map(|menu| {
                let mut anchored = anchored().snap_to_window_with_margin(px(8.0));
                if let Some(anchor) = this.anchor {
                    anchored = anchored.anchor(anchor)
                }
                anchored = anchored.position(*state.pos.borrow());

                let mut element = deferred(anchored.child(div().occlude().child(menu.clone())))
                    .with_priority(1)
                    .into_any();

                menu_layout_id = Some(element.request_layout(window, cx));
                element
            });

            let mut child = this
                .child_builder
                .take()
                .map(|builder| (builder)(state.menu.borrow().is_some(), window, cx));

            let child_layout_id = child.as_mut().map(|child| child.request_layout(window, cx));

            let layout_id = window.request_layout(
                Style::default(),
                menu_layout_id.into_iter().chain(child_layout_id),
                cx,
            );

            (
                layout_id,
                RequestLayoutState {
                    child_element: child,
                    child_layout_id,
                    menu_element,
                },
            )
        })
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);

        if let Some(child) = request_layout.child_element.as_mut() {
            child.prepaint(window, cx);
        }

        if let Some(menu) = request_layout.menu_element.as_mut() {
            menu.prepaint(window, cx);
        }

        PrepaintState {
            hitbox,
            child_bounds: request_layout
                .child_layout_id
                .map(|layout_id| window.layout_bounds(layout_id)),
        }
    }

    fn paint(
            &mut self,
            id: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            _: Bounds<Pixels>,
            request_layout: &mut Self::RequestLayoutState,
            prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
        self.with_element_state(
            id.unwrap(),
            window,
            cx,
            |this, element_state, window, cx| {
                if let Some(mut child) = request_layout.child_element.take() {
                    child.paint(window, cx);
                }

                if let Some(mut menu) = request_layout.menu_element.take() {
                    menu.paint(window, cx);
                    return;
                }

                let Some(builder) = this.menu_builder.take() else {
                    return;
                };

                let attach = this.attach;
                let menu = element_state.menu.clone();
                let position = element_state.pos.clone();
                let child_bounds = prepaint.child_bounds;

                let trigger_button = this.trigger_button;
                let hitbox_id = prepaint.hitbox.id;
                window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble
                        && event.button == trigger_button
                        && hitbox_id.is_hovered(window)
                    {
                        cx.stop_propagation();
                        window.prevent_default();

                        let new_menu = (builder)(window, cx);
                        let menu2 = menu.clone();
                        let previous_focus_handle = window.focused(cx);

                        window
                            .subscribe(&new_menu, cx, move |modal, _: &DismissEvent, window, cx| {
                                if modal.focus_handle(cx).contains_focused(window, cx)
                                    && let Some(previous_focus_handle) =
                                        previous_focus_handle.as_ref()
                                {
                                    window.focus(previous_focus_handle);
                                }
                                *menu2.borrow_mut() = None;
                                window.refresh();
                            })
                            .detach();
                        window.focus(&new_menu.focus_handle(cx));
                        *menu.borrow_mut() = Some(new_menu);
                        *position.borrow_mut() = if let Some(child_bounds) = child_bounds {
                            if let Some(attach) = attach {
                                child_bounds.corner(attach)
                            } else {
                                window.mouse_position()
                            }
                        } else {
                            window.mouse_position()
                        };
                        window.refresh();
                    }
                });
            },
        )
    }
}

impl<M: ManagedView> IntoElement for CtxMenu<M> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
