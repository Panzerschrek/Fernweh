use super::{math_types::*, ogl_common::*};

pub struct FieldBorderVisualizer
{
	vertex_buffer: glium::VertexBuffer<Vertex>,
	index_buffer: glium::IndexBuffer<u16>,
	program: glium::Program,
}

impl FieldBorderVisualizer
{
	pub fn new(display: &glium::Display) -> Self
	{
		let vertices = [
			Vertex {
				position: [0.0, 0.0, 0.0],
			},
			Vertex {
				position: [0.0, 0.0, 1.0],
			},
			Vertex {
				position: [0.0, 1.0, 0.0],
			},
			Vertex {
				position: [0.0, 1.0, 1.0],
			},
			Vertex {
				position: [1.0, 0.0, 0.0],
			},
			Vertex {
				position: [1.0, 0.0, 1.0],
			},
			Vertex {
				position: [1.0, 1.0, 0.0],
			},
			Vertex {
				position: [1.0, 1.0, 1.0],
			},
		];

		let indices = [0, 1, 0, 2, 1, 3, 2, 3, 4, 5, 4, 6, 5, 7, 6, 7, 0, 4, 2, 6, 3, 7, 1, 5];

		let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
		let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &indices).unwrap();

		let program = glium::Program::from_source(display, SHADER_V, SHADER_F, None).unwrap();

		Self {
			vertex_buffer,
			index_buffer,
			program,
		}
	}

	pub fn visualize<S: glium::Surface>(&self, surface: &mut S, view_matrix: &Mat4f, size: &[u32; 3])
	{
		let matrix = view_matrix * Mat4f::from_nonuniform_scale(size[0] as f32, size[1] as f32, size[2] as f32);

		let uniforms = glium::uniform! {
			matrix: make_uniform_matrix(&matrix)
		};

		surface
			.draw(
				&self.vertex_buffer,
				&self.index_buffer,
				&self.program,
				&uniforms,
				&get_default_drawing_params(),
			)
			.unwrap();
	}
}

#[derive(Copy, Clone)]
struct Vertex
{
	position: [f32; 3],
}

glium::implement_vertex!(Vertex, position);

const SHADER_V: &str = r#"
	#version 430
	uniform mat4 matrix;
	in vec3 position;
	void main() {
		gl_Position = vec4(position, 1.0) * matrix;
	}
"#;

const SHADER_F: &str = r#"
	#version 430
	in vec4 vColor;
	out vec4 f_color;
	void main() {
		f_color = vec4(0.8, 0.8, 0.8, 1.0);
	}
"#;
