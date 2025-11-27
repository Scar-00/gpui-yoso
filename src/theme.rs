use gpui::*;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fs, io, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse theme: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io: {0}")]
    Io(#[from] io::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub background: Hsla,
    pub foreground: Hsla,
    pub muted_background: Hsla,
    pub muted: Hsla,
    pub secondary: Hsla,
    pub text: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub hint: Hsla,
    pub selection: Hsla,
    pub border: Hsla,
}

impl Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: rgb(0x191724).into(),
            foreground: rgb(0x1f1d2e).into(),
            muted_background: rgb(0x26233a).into(),
            muted: rgb(0x6e6a86).into(),
            secondary: rgb(0x908caa).into(),
            text: rgb(0xe0def4).into(),
            error: rgb(0xeb6f92).into(),
            warning: rgb(0xf6c177).into(),
            hint: rgb(0xc4a7e7).into(),
            selection: rgb(0x403d52).into(),
            border: rgb(0x524f67).into(),
        }
    }
}

impl Theme {
    pub fn from_file(path: impl AsRef<Path>) -> Result<(String, Self)> {
        let content = fs::read_to_string(path.as_ref())?;
        Ok((
            path.as_ref().display().to_string(),
            serde_json::from_str(&content)?,
        ))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let str = serde_json::to_string_pretty(self)?;
        fs::write(path, &str)?;
        Ok(())
    }
}

pub trait AppTheme {
    fn theme(&self) -> &Theme;
}

impl<T: AppContext + Borrow<App>> AppTheme for T {
    fn theme(&self) -> &Theme {
        Theme::global((*self).borrow())
    }
}

pub fn init(cx: &mut App) {
    cx.set_global(Theme::default());
}

pub fn set_theme(cx: &mut App, theme: Theme) {
    if !cx.has_global::<Theme>() {
        init(cx);
    }
    cx.update_global(|active_theme, _| {
        *active_theme = theme;
    });
}
