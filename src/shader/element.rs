use super::init::ShaderContext;
use super::render::RenderContext;
use crate::{prelude::*, shader::render::IntoBuffer};
use gpui::{prelude::*, *};
use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};

pub struct ShaderOptions {
    clear_color: Hsla,
    size: [f32; 2],
}

impl ShaderOptions {
    pub fn new() -> Self {
        Self {
            clear_color: Hsla::default(),
            size: [0.0; 2],
        }
    }
}

pub struct Shader<T: IntoBuffer> {
    context: Option<RenderContext<T>>,
    image: Option<Arc<RenderImage>>,
}

impl<T: IntoBuffer + 'static> Shader<T> {
    pub fn new<C: AppContext + BorrowMut<App>>(
        cx: &mut C,
        src: ShaderSrc,
        data: &T,
    ) -> C::Result<Entity<Self>> {
        let app = (*cx).borrow_mut();
        let context = ShaderContext::update_global(app, |context, _| {
            RenderContext::new(context, src)
                .inspect_err(|e| {
                    tracing::error!("{e}");
                })
                .ok()
        });
        cx.new(|cx| {
            let mut shader = Shader {
                context: context,
                image: None,
            };
            shader.render_image(cx, data);
            shader
        })
    }

    pub fn rerender<C: AppContext + Borrow<App>>(&mut self, data: &T, cx: &mut C) {
        self.render_image(cx, data);
    }

    fn render_image<C: AppContext + Borrow<App>>(&mut self, cx: &mut C, data: &T) {
        let app = (*cx).borrow();
        let ctx = ShaderContext::global(app);
        if let Some(render) = &self.context {
            self.image = render
                .render_image(ctx, data)
                .inspect_err(|e| {
                    tracing::error!("{e}");
                })
                .map(|image| {
                    use image::Frame;
                    Arc::new(RenderImage::new([Frame::new(image)]))
                })
                .ok();
        }
    }
}

impl<T: IntoBuffer + 'static> Render for Shader<T> {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        if let Some(image) = self.image.clone() {
            canvas(
                |_, _, _| (),
                |bounds, _, window, _| {
                    window.on_mouse_event(move |ev: &MouseDownEvent, _, _, _| {
                        if ev.button == MouseButton::Left && bounds.contains(&ev.position) {
                            //dbg!(&bounds);
                            //dbg!(&ev.position);
                            println!("{:?}", ev.position - bounds.origin);
                        }
                    });
                    _ = window.paint_image(bounds, Default::default(), image, 0, false);
                },
            )
            .size_full()
            .into_any()
        } else {
            div().into_any()
        }
    }
}
