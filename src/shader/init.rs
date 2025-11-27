use gpui::{App, Global};
use pollster::FutureExt as _;
use shaderc::Compiler;
use std::error::Error;
use wgpu::*;

#[allow(dead_code)]
pub mod shader {
    pub const VS: &str = r"
        #version 450

        const vec2 positions[3] = vec2[3](
            vec2(0.0, 0.5),
            vec2(-0.5, -0.5),
            vec2(0.5, -0.5)
        );

        void main() {
            gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
        }
    ";

    pub const FS: &str = r"
        #version 450

        layout(binding = 0) uniform UserInput
        {
            vec4 color;
        }user_input;

        layout(location=0) out vec4 f_color;

        void main() {
            f_color = user_input.color;
        }
    ";

    pub const WGSL: &str = r"
        struct VertexOutput {
            @builtin(position) clip_position: vec4<f32>,
        };

        struct UserData {
            @location(0) color: vec4<f32>
        };

        @group(0) @binding(0)
        var<uniform> user_data: UserData;

        @vertex
        fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
            let pos = array(
                vec2f(0.0, 0.5),
                vec2f(-0.5, -0.5),
                vec2f(0.5, -0.5)
            );

            var out: VertexOutput;
            out.clip_position = vec4<f32>(pos[index], 0.0, 1.0);
            return out;
        }

        @fragment
        fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
            return user_data.color;
        }
    ";
}

pub struct ShaderContext {
    pub compiler: Compiler,
    pub device: Device,
    pub queue: Queue,
}

impl Global for ShaderContext {}

pub fn init(cx: &mut App) {
    let context = init_internal().block_on().expect("init shader context");
    cx.set_global(context);
}

async fn init_internal() -> Result<ShaderContext, Box<dyn Error>> {
    let instance = Instance::new(&InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await?;

    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: Default::default(),
            trace: Trace::Off,
        })
        .await?;

    Ok(ShaderContext {
        compiler: Compiler::new()?,
        device,
        queue,
    })
}
