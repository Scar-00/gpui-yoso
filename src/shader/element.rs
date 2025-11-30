use super::init::ShaderContext;
use super::render::RenderContext;
use crate::{prelude::*, shader::render::IntoBuffer};
use gpui::{prelude::*, *};
use std::{
    borrow::{Borrow, BorrowMut}, rc::Rc, sync::Arc
};

pub struct ShaderOptions<T: IntoBuffer> {
    pub(crate) src: ShaderSrc,
    pub(crate) clear_color: Hsla,
    pub(crate) size: [u32; 2],
    on_click: Option<Rc<dyn Fn(Point<Pixels>, &mut Window, &mut App, WeakEntity<Shader<T>>)>>,
}

impl<T: IntoBuffer> ShaderOptions<T> {
    pub fn new(src: impl Into<ShaderSrc>) -> Self {
        Self {
            src: src.into(),
            clear_color: Hsla::default(),
            size: [0; 2],
            on_click: None,
        }
    }

    pub fn clear_color(self, clear_color: impl Into<Hsla>) -> Self {
        Self {
            clear_color: clear_color.into(),
            ..self
        }
    }

    pub fn size(self, size: impl Into<[u32; 2]>) -> Self {
        Self {
            size: size.into(),
            ..self
        }
    }

    pub fn on_click(self, on_click: impl Fn(Point<Pixels>, &mut Window, &mut App, WeakEntity<Shader<T>>) + 'static) -> Self {
        Self{
            on_click: Some(Rc::new(on_click)),
            ..self
        }
    }
}

pub struct Shader<T: IntoBuffer> {
    context: Option<RenderContext<T>>,
    image: Option<Arc<RenderImage>>,
    on_click: Option<Rc<dyn Fn(Point<Pixels>, &mut Window, &mut App, WeakEntity<Self>)>>
}

impl<T: IntoBuffer + 'static> Shader<T> {
    pub fn new<C: AppContext + BorrowMut<App>>(
        window: &mut Window,
        cx: &mut C,
        options: ShaderOptions<T>,
        data: &T,
    ) -> C::Result<Entity<Self>> {
        let app = (*cx).borrow_mut();
        let on_click = options.on_click.clone();
        let context = ShaderContext::update_global(app, |context, _| {
            RenderContext::new(context, options)
                .inspect_err(|e| {
                    tracing::error!("{e}");
                })
                .ok()
        });
        cx.new(|cx| {
            let mut shader = Shader {
                context: context,
                image: None,
                on_click,
            };
            shader.render_image(window, cx, data);
            shader
        })
    }

    pub fn rerender<C: AppContext + Borrow<App>>(&mut self, data: &T, window: &mut Window, cx: &mut C) {
        self.render_image(window, cx, data);
    }

    fn render_image<C: AppContext + Borrow<App>>(&mut self, window: &mut Window, cx: &mut C, data: &T) {
        let app = (*cx).borrow();
        let ctx = ShaderContext::global(app);
        if let Some(render) = &self.context {
            if let Some(image) = self.image.take() {
                _ = window.drop_image(image);
            }
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.weak_entity();
        let entity1 = cx.weak_entity();
        let on_click = self.on_click.clone();
        let on_click1 = self.on_click.clone();
        if let Some(image) = self.image.clone() {
            let image_size = self.context.as_ref().unwrap().options.size;
            canvas(
                |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::BlockMouseExceptScroll),
                move |bounds, _, window, _| {
                    window.on_mouse_event(move |ev: &MouseDownEvent, _, window, cx| {
                        if ev.button == MouseButton::Left && bounds.contains(&ev.position) {
                            if let Some(on_click) = &on_click {
                                let x_scalar = px(image_size[0] as f32) / bounds.size.width;
                                let y_scalar = px(image_size[1] as f32) / bounds.size.height;
                                let mut point = ev.position - bounds.origin;
                                point.x *= x_scalar;
                                point.y *= y_scalar;
                                on_click(
                                    point,
                                    window,
                                    cx,
                                    entity.clone(),
                                );
                            }
                        }
                    });
                    window.on_mouse_event(move |ev: &MouseMoveEvent, _, window, cx| {
                        if bounds.contains(&ev.position) {
                            if let Some(on_click) = &on_click1 {
                                let x_scalar = px(image_size[0] as f32) / bounds.size.width;
                                let y_scalar = px(image_size[1] as f32) / bounds.size.height;
                                let mut point = ev.position - bounds.origin;
                                point.x *= x_scalar;
                                point.y *= y_scalar;
                                on_click(
                                    point,
                                    window,
                                    cx,
                                    entity1.clone(),
                                );
                            }
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
