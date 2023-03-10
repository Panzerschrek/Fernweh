use super::{math_types::*, ogl_common::*, vector_field};

pub struct VectorFieldVisualizer
{
	program: glium::Program,
}

impl VectorFieldVisualizer
{
	pub fn new(display: &glium::Display) -> Self
	{
		let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
		Self { program }
	}

	pub fn visualize<S: glium::Surface>(
		&self,
		surface: &mut S,
		vector_field: &vector_field::VectorField,
		view_matrix: &Mat4f,
		base_color: [f32; 3],
	)
	{
		let field_size = vector_field.get_size();

		let uniforms = glium::uniform! {
			view_matrix: make_uniform_matrix(view_matrix),
			field_size: field_size,
			field_data: vector_field.get_buffer(),
			base_color: base_color,
		};

		surface
			.draw(
				glium::vertex::EmptyVertexAttributes {
					len: (field_size[0] * field_size[1] * field_size[2] * 2) as usize,
				},
				glium::index::IndicesSource::NoIndices {
					primitives: glium::index::PrimitiveType::LinesList,
				},
				&self.program,
				&uniforms,
				&get_default_drawing_params(),
			)
			.unwrap();
	}
}

const VERTEX_SHADER: &str = r#"
	#version 430

	uniform uvec3 field_size;
	uniform mat4 view_matrix;
	uniform vec3 base_color;

	layout(std430) buffer field_data
	{
		vec4 vecs[];
	};

	out vec3 f_color;

	void main()
	{
		uint cell_id = uint(gl_VertexID) / 2;
		uint layer_size = field_size.y * field_size.x;
		uint z = cell_id / layer_size;
		uint z_id = cell_id - z * layer_size;
		uint y = z_id / field_size.x;
		uint x = z_id % field_size.x;

		vec3 position = vec3( uvec3(x, y, z) ) + vec3(0.5, 0.5, 0.5);

		float arrow_tip_factor = float(gl_VertexID & 1);
		vec3 vec = vecs[ cell_id ].xyz;
		float vec_len = length(vec);
		vec3 vec_clamped = vec * ( min(vec_len, 1.5) / max(vec_len, 0.0000001) );

		position += vec_clamped * arrow_tip_factor;

		gl_Position = vec4(position, 1.0) * view_matrix;
		f_color = base_color * (0.02 + (1.0 - arrow_tip_factor) * vec_len);
	}
"#;

const FRAGMENT_SHADER: &str = r#"
	#version 430
	in vec3 f_color;
	out vec4 color;
	void main()
	{
		color = vec4(f_color, 1.0);
	}
"#;
