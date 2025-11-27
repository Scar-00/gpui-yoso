pub mod element;
//pub mod test;
mod init;
mod render;
pub use render::ShaderSrc;

pub fn init(cx: &mut gpui::App) {
    init::init(cx)
}
