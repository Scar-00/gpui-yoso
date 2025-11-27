pub mod linux;
pub mod mac;
pub mod windows;

use crate::prelude::*;
use gpui::{prelude::*, *};

trait WindowTopBarImpl {
    fn render(&self, window: &mut Window, cx: &mut Context<WindowTopBar>) -> AnyElement;
}

pub struct WindowTopBar(Box<dyn WindowTopBarImpl>);

impl WindowTopBar {
    pub fn new<C: AppContext>(cx: &mut C) -> C::Result<Entity<Self>> {
        cx.new(|_| {
            let window = windows::WindowsTopBar {};
            WindowTopBar(Box::new(window))
        })
    }

    pub fn top() -> AnyElement {
        div().into_any_element()
    }
}

impl Render for WindowTopBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.0.render(window, cx)
    }
}
