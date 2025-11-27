#![allow(clippy::type_complexity)]

pub mod button;
pub mod checkbox;
pub mod input;
pub mod label;
pub mod layout;
pub mod navigation;
pub mod overlay;
pub mod shader;
pub mod spinner;
pub mod theme;
pub mod util;
pub mod window;

pub mod prelude {
    pub use crate::button::*;
    pub use crate::checkbox::Checkbox;
    use crate::input;
    pub use crate::input::TextInput;
    pub use crate::label::*;
    use crate::layout;
    pub use crate::navigation::tab_bar::{Tab, TabBar};
    pub use crate::overlay::{
        menu::CtxMenu,
        menu_builder::{MenuBuilder, MenuListItem},
        popup::Popup,
        tooltip::Tooltip,
    };
    pub use crate::shader::{
        self,
        element::{Shader, ShaderOptions},
        ShaderSrc,
    };
    pub use crate::spinner::*;
    pub use crate::theme::{self, AppTheme, Theme};
    pub use crate::util::{self, *};
    pub use crate::window::WindowTopBar;
    pub use gpui;
    pub use layout::divider::{self, Divider};

    pub fn init_all(cx: &mut gpui::App) {
        theme::init(cx);
        input::init(cx);
        shader::init(cx);
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, *};
    use gpui::{
        div, px, AppContext, Application, Context, Entity, IntoElement, ParentElement, Render,
        Styled, Window,
    };

    struct TestMain {
        value: i32,
        input: Entity<TextInput>,
    }

    impl Render for TestMain {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .text_size(px(16.0))
                .flex()
                .flex_col()
                .gap_4()
                .size_full()
                .bg(cx.theme().background)
                .p_2()
                .child(
                    Spinner::new(
                        "test",
                        self.value.to_string(),
                        cx.listener(|this, _, _, _| {
                            this.value += 1;
                        }),
                        cx.listener(|this, _, _, _| {
                            this.value -= 1;
                        }),
                    )
                    .border_2()
                    .border_color(cx.theme().border)
                    .rounded_lg()
                    .w_40(),
                )
                .child(
                    div()
                        .w_40()
                        .border_2()
                        .border_color(cx.theme().border)
                        .rounded_lg()
                        .text_color(cx.theme().text)
                        .child(self.input.clone()), //.child(Tooltip::new("test")),
                )
        }
    }

    #[gpui::test]
    fn test() {
        Application::new().with_assets(util::Assets).run(|cx| {
            init_all(cx);
            _ = cx.open_window(Default::default(), |_, cx| {
                cx.new(|cx| TestMain {
                    value: 0,
                    input: cx.new(|cx| {
                        let focus = cx.focus_handle();
                        TextInput::new(focus, None, None, None)
                    }),
                })
            });
            cx.activate(true);
        })
    }
}
