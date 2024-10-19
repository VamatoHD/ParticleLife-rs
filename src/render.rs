use miniquad::*;

use std::path::Path;
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type UpdateFn = fn(stage: &mut Stage);
type RenderFn = fn(stage: &mut Stage);

#[repr(C)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

pub struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx:Box<dyn RenderingBackend>,
    particles: usize,

    update_fn: UpdateFn,
    render_fn: RenderFn,
}

impl Stage {
    pub fn new(
        particles: usize,
        update_fn: UpdateFn,
        render_fn: RenderFn,
    ) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let vertex_shader =
            read_file("shaders/vertex_shader.glsl").expect("Failed to load vertex shader");
        let fragment_shader =
            read_file("shaders/fragment_shader.glsl").expect("Failed to load fragment shader");

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for i in 0..particles {
            let base_index = i as u16 * 3;
            for _ in 0..3 {
                vertices.push(Vertex {
                    pos: [0., 0.],
                    color: [1., 1., 1., 1.],
                });
            }

            indices.push(base_index);
            indices.push(base_index + 1);
            indices.push(base_index + 2);
        }

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
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
            particles,
            update_fn,
            render_fn
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        (self.update_fn)(self);
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        (self.render_fn)(self);
        
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}
