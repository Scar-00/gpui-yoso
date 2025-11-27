use std::borrow::{Borrow, Cow};

use crate::prelude::*;
use gpui::{prelude::*, *};

pub struct Assets;

impl Assets {
    const ARROW_UP: &[u8] = include_bytes!("../assets/arrow_up.svg");
    const ARROW_DOWN: &[u8] = include_bytes!("../assets/arrow_down.svg");
    const CHEVRON_UP: &[u8] = include_bytes!("../assets/chevron_up.svg");
    const CHEVRON_DOWN: &[u8] = include_bytes!("../assets/chevron_down.svg");
    const CHECKBOX_INNER: &[u8] = include_bytes!("../assets/checkbox_inner.svg");
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "arrow_up" => Some(Cow::Borrowed(Self::ARROW_UP)),
            "arrow_down" => Some(Cow::Borrowed(Self::ARROW_DOWN)),
            "chevron_up" => Some(Cow::Borrowed(Self::CHEVRON_UP)),
            "chevron_down" => Some(Cow::Borrowed(Self::CHEVRON_DOWN)),
            "checkbox-inner" => Some(Cow::Borrowed(Self::CHECKBOX_INNER)),
            _ => None,
        })
    }

    fn list(&self, _: &str) -> Result<Vec<SharedString>> {
        todo!()
    }
}

pub fn v_flex() -> Div {
    div().flex().flex_col()
}

pub fn h_flex() -> Div {
    div().flex().flex_row()
}

pub fn border(cx: impl AppContext + Borrow<App>) -> Div {
    div().border_1().border_color(cx.theme().border)
}

pub fn border_round(cx: impl AppContext + Borrow<App>) -> Div {
    div()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_sm()
}
