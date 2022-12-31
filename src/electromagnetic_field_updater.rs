use super::electromagnetic_field::*;

pub struct ElectromagneticFieldUpdater
{
	shader: glium::program::ComputeShader,
}

impl ElectromagneticFieldUpdater
{
	pub fn new(display: &glium::Display) -> Self
	{
		let shader = glium::program::ComputeShader::from_source(display, SHADER).unwrap();
		Self { shader }
	}

	pub fn update(&self, field: &mut ElectromagneticField, time_delta_s: f32)
	{
		assert_eq!(field.electric_field.get_size(), field.magnetic_field.get_size());

		let field_size = field.electric_field.get_size();

		let uniforms = glium::uniform! {
			dt: time_delta_s,
			field_size: field_size,
			field_data: field.electric_field.get_buffer(),
		};

		self.shader
			.execute(uniforms, field_size[0], field_size[1], field_size[2]);
	}
}

const SHADER: &str = r#"
	#version 430
	layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

	uniform float dt;
	uniform uvec3 field_size;
	layout(std430) buffer field_data
	{
		vec4 vecs[];
	};

	void main()
	{
		uint address = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y * field_size.x + gl_GlobalInvocationID.z * (field_size.x * field_size.y);
		vec3 vec = vecs[address].xyz;
		vec *= 1.0 + dt;
		vecs[address] = vec4(vec, 0.0);
	}
"#;
