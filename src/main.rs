use miniquad::*;
mod file;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let vertex_shader =
            file::read("shaders/vertex_shader.glsl").expect("Failed to load vertex shader");
        let fragment_shader =
            file::read("shaders/fragment_shader.glsl").expect("Failed to load fragment shader");

        #[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [1., 1., 1., 1.] },
            Vertex { pos : [  0.5, -0.5 ], color: [1., 1., 1., 1.] },
            Vertex { pos : [  0.0,  0.5 ], color: [1., 1., 1., 1.] },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: &vertex_shader,
                    fragment: &fragment_shader,
                },
                ShaderMeta {
                    images: vec![],
                    uniforms: UniformBlockLayout { uniforms: vec![] },
                },
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        self.ctx.draw(0, 3, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new()));
}
