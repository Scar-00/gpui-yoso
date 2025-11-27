use image::{ImageBuffer, Rgba};
use shaderc::ShaderKind;
use std::{error::Error, marker::PhantomData, path::PathBuf, usize};
use wgpu::*;

use crate::shader::init::ShaderContext;

pub enum ShaderSrc {
    WgslStatic {
        src: &'static str,
        vs_entry: &'static str,
        fs_entry: &'static str,
    },
    Wgsl {
        src: PathBuf,
        vs_entry: String,
        fs_entry: String,
    },
    GlslStatic {
        vs_src: &'static str,
        fs_src: &'static str,
        vs_entry: &'static str,
        fs_entry: &'static str,
    },
    Glsl {
        vs_src: PathBuf,
        fs_src: PathBuf,
        vs_entry: String,
        fs_entry: String,
    },
}

impl ShaderSrc {
    fn create_shader(&self, cx: &mut ShaderContext) -> Result<Shader, Box<dyn Error>> {
        match self {
            Self::WgslStatic {
                src,
                vs_entry,
                fs_entry,
            } => create_wgsl_shader(cx, src, vs_entry, fs_entry),
            Self::Wgsl {
                src,
                vs_entry,
                fs_entry,
            } => create_wgsl_shader(cx, &std::fs::read_to_string(src)?, vs_entry, fs_entry),
            Self::GlslStatic {
                vs_src,
                fs_src,
                vs_entry,
                fs_entry,
            } => create_glsl_shader(cx, vs_src, vs_entry, fs_src, fs_entry),
            Self::Glsl {
                vs_src,
                fs_src,
                vs_entry,
                fs_entry,
            } => create_glsl_shader(
                cx,
                &std::fs::read_to_string(vs_src)?,
                vs_entry,
                &std::fs::read_to_string(fs_src)?,
                fs_entry,
            ),
        }
    }
}

pub struct RenderContext<T: IntoBuffer> {
    pipeline: RenderPipeline,
    src_texture: Texture,
    bind_group: BindGroup,
    uniform_buffer: Buffer,
    out_buffer: Buffer,
    _data: PhantomData<T>,
}

impl<T: IntoBuffer> RenderContext<T> {
    pub fn new(cx: &mut ShaderContext, src: ShaderSrc) -> Result<Self, Box<dyn Error>> {
        let src_texture_descriptor = TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let src_texture = cx.device.create_texture(&src_texture_descriptor);

        let out_buffer_descriptor = BufferDescriptor {
            label: None,
            size: 512 * 512 * 4,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        };
        let out_buffer = cx.device.create_buffer(&out_buffer_descriptor);

        let shader = src.create_shader(cx)?;
        let (pipeline, uniform_buffer, bind_group) =
            create_render_pass::<T>(cx, &shader, &src_texture)?;
        Ok(Self {
            pipeline,
            uniform_buffer,
            bind_group,
            src_texture,
            out_buffer,
            _data: PhantomData {},
        })
    }

    pub fn render_image(
        &self,
        cx: &ShaderContext,
        user_data: &T,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn Error>> {
        render_image(cx, self, user_data)
    }
}

pub enum Shader {
    Wgsl {
        module: ShaderModule,
        vs_entry: String,
        fs_entry: String,
    },
    Glsl {
        vs: ShaderModule,
        vs_entry: String,
        fs: ShaderModule,
        fs_entry: String,
    },
}

impl Shader {
    fn vs_module(&self) -> &ShaderModule {
        match self {
            Self::Wgsl { module, .. } => module,
            Self::Glsl { vs, .. } => vs,
        }
    }

    fn fs_module(&self) -> &ShaderModule {
        match self {
            Self::Wgsl { module, .. } => module,
            Self::Glsl { fs, .. } => fs,
        }
    }

    fn vs_entry(&self) -> &str {
        match self {
            Self::Wgsl { vs_entry, .. } => &vs_entry,
            Self::Glsl { vs_entry, .. } => &vs_entry,
        }
    }

    fn fs_entry(&self) -> &str {
        match self {
            Self::Wgsl { fs_entry, .. } => &fs_entry,
            Self::Glsl { fs_entry, .. } => &fs_entry,
        }
    }
}

pub fn create_glsl_shader(
    cx: &mut ShaderContext,
    vs_src: &str,
    vs_entry: &str,
    fs_src: &str,
    fs_entry: &str,
) -> Result<Shader, Box<dyn Error>> {
    let vs = cx.compiler.compile_into_spirv(
        vs_src,
        ShaderKind::Vertex,
        "internal-vs",
        vs_entry,
        None,
    )?;

    let vs = wgpu::util::make_spirv(vs.as_binary_u8());

    let vs_module = cx.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("vs-shader"),
        source: vs,
    });

    let fs = cx.compiler.compile_into_spirv(
        fs_src,
        ShaderKind::Fragment,
        "internal-vs",
        fs_entry,
        None,
    )?;

    let fs = wgpu::util::make_spirv(fs.as_binary_u8());

    let fs_module = cx.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("fs-shader"),
        source: fs,
    });
    Ok(Shader::Glsl {
        vs: vs_module,
        vs_entry: vs_entry.to_string(),
        fs: fs_module,
        fs_entry: fs_entry.to_string(),
    })
}

pub fn create_wgsl_shader<'a>(
    cx: &mut ShaderContext,
    src: &str,
    vs_entry: impl ToString,
    fs_entry: impl ToString,
) -> Result<Shader, Box<dyn Error>> {
    let module = cx.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("wgsl-shader"),
        source: ShaderSource::Wgsl(src.into()),
    });
    Ok(Shader::Wgsl {
        module,
        vs_entry: vs_entry.to_string(),
        fs_entry: fs_entry.to_string(),
    })
}

pub fn create_render_pass<T: IntoBuffer>(
    cx: &ShaderContext,
    shader: &Shader,
    src_texture: &Texture,
) -> Result<(RenderPipeline, Buffer, BindGroup), Box<dyn Error>> {
    let bind_group_layout = cx
        .device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("bind-group-layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

    let buffer = cx.device.create_buffer(&BufferDescriptor {
        label: Some("uniform-buffer"),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        size: std::mem::size_of::<T>() as u64,
        mapped_at_creation: false,
    });

    let bind_group = cx.device.create_bind_group(&BindGroupDescriptor {
        label: Some("bind-group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            //resource: cx.out_buffer.as_entire_binding(),
            resource: buffer.as_entire_binding(),
        }],
    });

    let layout = cx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("pipeline-layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = cx.device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("compute-pipeline"),
        layout: Some(&layout),
        vertex: VertexState {
            module: &shader.vs_module(),
            entry_point: Some(&shader.vs_entry()),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        },
        fragment: Some(FragmentState {
            module: &shader.fs_module(),
            entry_point: Some(&shader.fs_entry()),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState {
                format: src_texture.format(),
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });

    Ok((pipeline, buffer, bind_group))
}

pub fn render_image<T: IntoBuffer>(
    cx: &ShaderContext,
    render: &RenderContext<T>,
    user_data: &T,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn Error>> {
    cx.queue
        .write_buffer(&render.uniform_buffer, 0, user_data.as_u8_slice());
    cx.queue.submit([]);

    let mut encoder = cx
        .device
        .create_command_encoder(&CommandEncoderDescriptor { label: None });

    let compute_pass_descriptor = RenderPassDescriptor {
        label: Some("renderpass-descriptor"),
        timestamp_writes: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &render.src_texture.create_view(&Default::default()),
            depth_slice: None,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
    };

    {
        let mut render_pass = encoder.begin_render_pass(&compute_pass_descriptor);
        render_pass.set_pipeline(&render.pipeline);
        render_pass.set_bind_group(0, &render.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            aspect: TextureAspect::All,
            texture: &render.src_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
        },
        TexelCopyBufferInfo {
            buffer: &render.out_buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 512),
                rows_per_image: Some(512),
            },
        },
        render.src_texture.size(),
    );

    cx.queue.submit([encoder.finish()]);

    let image = {
        let data = render.out_buffer.slice(..);
        data.map_async(MapMode::Read, |result| {
            if let Err(e) = result {
                tracing::error!("{e}");
            }
        });
        cx.device.poll(PollType::wait())?;
        let data = render.out_buffer.get_mapped_range(..);
        use image::{ImageBuffer, Rgba};
        ImageBuffer::<Rgba<u8>, _>::from_raw(512, 512, data.to_vec())
            .expect("failed to create image")
    };
    render.out_buffer.unmap();
    Ok(image)
}

pub trait IntoBuffer {
    fn as_u8_slice(&self) -> &[u8];
}

/*pub trait VertexData: Sized {
    const VERTEX_ATTRIBUTES: &[VertexAttribute];

    fn descriptor() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}*/

impl<T> IntoBuffer for [T] {
    fn as_u8_slice(&self) -> &[u8] {
        let base = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts(base, size * self.len()) };
        slice
    }
}

impl<T, const N: usize> IntoBuffer for [T; N] {
    fn as_u8_slice(&self) -> &[u8] {
        let base = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts(base, size * self.len()) };
        slice
    }
}

impl<T> IntoBuffer for Vec<T> {
    fn as_u8_slice(&self) -> &[u8] {
        let base = self.as_ptr() as *const u8;
        let size = std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts(base, size * self.len()) };
        slice
    }
}
