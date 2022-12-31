use super::electromagnetic_field::*;

pub struct ElectromagneticFieldUpdater
{
	shader_electric_field_update: glium::program::ComputeShader,
	shader_magnetic_field_update: glium::program::ComputeShader,
}

impl ElectromagneticFieldUpdater
{
	pub fn new(display: &glium::Display) -> Self
	{
		Self {
			shader_electric_field_update: glium::program::ComputeShader::from_source(display, SHADER_ELECTRIC_UPDATE)
				.unwrap(),
			shader_magnetic_field_update: glium::program::ComputeShader::from_source(display, SHADER_MAGNETIC_UPDATE)
				.unwrap(),
		}
	}

	pub fn update(&self, field: &mut ElectromagneticField, time_delta_s: f32)
	{
		assert_eq!(field.electric_field.get_size(), field.magnetic_field.get_size());

		let field_size = field.electric_field.get_size();

		let uniforms = glium::uniform! {
			dt: time_delta_s,
			field_size: field_size,
			electric_field_data: field.electric_field.get_buffer(),
			magnetic_field_data: field.magnetic_field.get_buffer(),
		};

		self.shader_electric_field_update
			.execute(uniforms, field_size[0], field_size[1], field_size[2]);
		self.shader_magnetic_field_update
			.execute(uniforms, field_size[0], field_size[1], field_size[2]);
	}
}

const SHADER_ELECTRIC_UPDATE: &str = r#"
	#version 430
	layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

	uniform float dt;
	uniform uvec3 field_size;
	layout(std430) buffer electric_field_data
	{
		vec4 electric_vecs[];
	};
	layout(std430) buffer magnetic_field_data
	{
		vec4 magnetic_vecs[];
	};

	void main()
	{
		uvec3 coord = gl_GlobalInvocationID;

		uint layer_size = field_size.x * field_size.y;
		uint address_center  = coord.x + coord.y * field_size.x + coord.z * layer_size;
		uint address_x_minus = (max(coord.x, 1) - 1) + coord.y * field_size.x + coord.z * layer_size;
		uint address_y_minus = coord.x + (max(coord.y, 1) - 1) * field_size.x + coord.z * layer_size;
		uint address_z_minus = coord.x + coord.y * field_size.x + (max(coord.z, 1) - 1) * layer_size;

		vec3 vec_center  = electric_vecs[address_center ].xyz;
		vec3 vec_x_minus = electric_vecs[address_x_minus].xyz;
		vec3 vec_y_minus = electric_vecs[address_y_minus].xyz;
		vec3 vec_z_minus = electric_vecs[address_z_minus].xyz;

		vec3 x_derivative = vec_center - vec_x_minus;
		vec3 y_derivative = vec_center - vec_y_minus;
		vec3 z_derivative = vec_center - vec_z_minus;

		vec3 curl =
			vec3(
				y_derivative.z - z_derivative.y,
				z_derivative.x - x_derivative.z,
				x_derivative.y - y_derivative.x );

		vec3 magnetic_field_change = -curl * dt;
		magnetic_vecs[address_center] += vec4(magnetic_field_change, 0.0);
	}
"#;

const SHADER_MAGNETIC_UPDATE: &str = r#"
	#version 430
	layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

	uniform float dt;
	uniform uvec3 field_size;
	layout(std430) buffer electric_field_data
	{
		vec4 electric_vecs[];
	};
	layout(std430) buffer magnetic_field_data
	{
		vec4 magnetic_vecs[];
	};

	void main()
	{
		uvec3 coord = gl_GlobalInvocationID;

		uint layer_size = field_size.x * field_size.y;
		uint address_center  = coord.x + coord.y * field_size.x + coord.z * layer_size;
		uint address_x_minus = (max(coord.x, 1) - 1) + coord.y * field_size.x + coord.z * layer_size;
		uint address_y_minus = coord.x + (max(coord.y, 1) - 1) * field_size.x + coord.z * layer_size;
		uint address_z_minus = coord.x + coord.y * field_size.x + (max(coord.z, 1) - 1) * layer_size;

		vec3 vec_center  = magnetic_vecs[address_center ].xyz;
		vec3 vec_x_minus = magnetic_vecs[address_x_minus].xyz;
		vec3 vec_y_minus = magnetic_vecs[address_y_minus].xyz;
		vec3 vec_z_minus = magnetic_vecs[address_z_minus].xyz;

		vec3 x_derivative = vec_center - vec_x_minus;
		vec3 y_derivative = vec_center - vec_y_minus;
		vec3 z_derivative = vec_center - vec_z_minus;

		vec3 curl =
			vec3(
				y_derivative.z - z_derivative.y,
				z_derivative.x - x_derivative.z,
				x_derivative.y - y_derivative.x );

		vec3 electric_field_change = curl * dt;
		electric_vecs[address_center] += vec4(electric_field_change, 0.0);
	}
"#;
