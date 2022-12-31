use super::{
	electromagnetic_field::*, electromagnetic_field_updater, math_types::*, ogl_common::*, vector_field::*,
	vector_field_visualizer,
};

pub struct FieldsSimulator
{
	vertex_buffer: glium::VertexBuffer<Vertex>,
	index_buffer: glium::IndexBuffer<u16>,
	program: glium::Program,
	electromagnetic_field: ElectromagneticField,
	vector_field_visualizer: vector_field_visualizer::VectorFieldVisualizer,
	field_updater: electromagnetic_field_updater::ElectromagneticFieldUpdater,
}

impl FieldsSimulator
{
	pub fn new(display: &glium::Display) -> Self
	{
		let vertices = [
			Vertex {
				position: [2.0, -0.5, -0.5],
				color: [0.0, 1.0, 0.0, 1.0],
			},
			Vertex {
				position: [2.0, 0.0, 0.5],
				color: [0.0, 0.0, 1.0, 0.5],
			},
			Vertex {
				position: [1.7, 0.5, -0.5],
				color: [1.0, 0.0, 0.0, 0.25],
			},
			Vertex {
				position: [2.3, 0.0, 0.0],
				color: [1.0, 1.0, 1.0, 0.125],
			},
		];

		let indices = [0, 1, 2, 0, 2, 3];

		let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
		let index_buffer =
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

		let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

		let electromagnetic_field = create_test_field(&display);
		let vector_field_visualizer = vector_field_visualizer::VectorFieldVisualizer::new(&display);

		let field_updater = electromagnetic_field_updater::ElectromagneticFieldUpdater::new(&display);

		Self {
			vertex_buffer,
			index_buffer,
			program,
			electromagnetic_field,
			vector_field_visualizer,
			field_updater,
		}
	}

	pub fn update(&mut self, time_delta_s: f32)
	{
		let time_scaled = time_delta_s * 0.2;
		self.field_updater.update(&mut self.electromagnetic_field, time_scaled);
	}

	pub fn draw<S: glium::Surface>(&self, surface: &mut S, view_matrix: &Mat4f)
	{
		surface.clear_color(0.0, 0.0, 0.0, 0.0);
		surface.clear_depth(1.0);

		let uniforms = glium::uniform! {
			matrix: make_uniform_matrix(view_matrix)
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

		self.vector_field_visualizer.visualize(
			surface,
			&self.electromagnetic_field.electric_field,
			view_matrix,
			[1.0, 0.2, 1.0],
		);
	}
}

#[derive(Copy, Clone)]
struct Vertex
{
	position: [f32; 3],
	color: [f32; 4],
}

glium::implement_vertex!(Vertex, position, color);

const VERTEX_SHADER: &str = r#"
	#version 430
	uniform mat4 matrix;
	in vec3 position;
	in vec4 color;
	out vec4 vColor;
	void main() {
		gl_Position = vec4(position, 1.0) * matrix;
		vColor = color;
	}
"#;

const FRAGMENT_SHADER: &str = r#"
	#version 430
	in vec4 vColor;
	out vec4 f_color;
	void main() {
		f_color = vColor;
	}
"#;

fn create_test_field(display: &glium::Display) -> ElectromagneticField
{
	let size = [48 as u32, 32, 24];
	ElectromagneticField {
		electric_field: create_test_static_charge_field(display, size),
		magnetic_field: VectorField::new(display, size),
	}
}

fn create_test_static_charge_field(display: &glium::Display, size: [u32; 3]) -> VectorField
{
	let center = Vec3f::new(size[0] as f32, size[1] as f32, size[2] as f32) * 0.5;

	let inv_scale = 128.0 / center.magnitude2();

	let mut data = vec![[0.0; 4]; (size[0] * size[1] * size[2]) as usize];

	// Simulate electric field of point charge.
	for z in 0 .. size[2]
	{
		let dz = z as f32 - center.z;
		for y in 0 .. size[1]
		{
			let dy = y as f32 - center.y;
			for x in 0 .. size[0]
			{
				let dx = x as f32 - center.x;
				let vec = Vec3f::new(dx, dy, dz);
				let vec_square_len = vec.magnitude2();

				let field_vec = if vec_square_len <= 0.0
				{
					Vec3f::zero()
				}
				else
				{
					vec / (inv_scale * vec_square_len * vec_square_len.sqrt())
				};

				data[(x + y * size[0] + z * (size[0] * size[1])) as usize] = field_vec.extend(0.0).into();
			}
		}
	}

	VectorField::new_with_data(display, size, &data)
}
