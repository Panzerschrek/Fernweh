use super::{
	electromagnetic_field::*, electromagnetic_field_updater, field_border_visualizer::*, math_types::*,
	vector_field::*, vector_field_visualizer,
};

pub struct FieldsSimulator
{
	electromagnetic_field: ElectromagneticField,
	vector_field_visualizer: vector_field_visualizer::VectorFieldVisualizer,
	field_updater: electromagnetic_field_updater::ElectromagneticFieldUpdater,
	field_border_visualizer: FieldBorderVisualizer,
}

impl FieldsSimulator
{
	pub fn new(display: &glium::Display) -> Self
	{
		let electromagnetic_field = create_test_wave_field(&display);
		let vector_field_visualizer = vector_field_visualizer::VectorFieldVisualizer::new(&display);

		let field_updater = electromagnetic_field_updater::ElectromagneticFieldUpdater::new(&display);

		let field_border_visualizer = FieldBorderVisualizer::new(display);

		Self {
			electromagnetic_field,
			vector_field_visualizer,
			field_updater,
			field_border_visualizer,
		}
	}

	pub fn update(&mut self, time_delta_s: f32)
	{
		let time_scaled = time_delta_s;
		for _i in 0 .. 4
		{
			self.field_updater.update(&mut self.electromagnetic_field, time_scaled);
		}
	}

	pub fn draw<S: glium::Surface>(&self, surface: &mut S, view_matrix: &Mat4f)
	{
		surface.clear_color(0.0, 0.0, 0.0, 0.0);
		surface.clear_depth(1.0);

		self.vector_field_visualizer.visualize(
			surface,
			&self.electromagnetic_field.electric_field,
			view_matrix,
			ELECTRIC_FIELD_BASE_COLOR,
		);
		self.vector_field_visualizer.visualize(
			surface,
			&self.electromagnetic_field.magnetic_field,
			view_matrix,
			MAGNETIC_FIELD_BASE_COLOR,
		);

		self.field_border_visualizer.visualize(
			surface,
			view_matrix,
			&self.electromagnetic_field.electric_field.get_size(),
		);
	}
}

const ELECTRIC_FIELD_BASE_COLOR: [f32; 3] = [0.5, 0.1, 0.1];
const MAGNETIC_FIELD_BASE_COLOR: [f32; 3] = [0.1, 0.1, 0.5];

fn create_test_wave_field(display: &glium::Display) -> ElectromagneticField
{
	let size = [72 as u32, 72, 192];

	let len = (size[0] * size[1] * size[2]) as usize;
	let mut electric_data = vec![[0.0; 4]; len];
	let mut magnetic_data = vec![[0.0; 4]; len];

	let center = Vec3f::new(size[0] as f32 * 0.5, size[1] as f32 * 0.5, size[2] as f32 * 0.25);
	let frequency_mul_2pi = (2.0 * std::f32::consts::PI) / 12.0;

	for z in 0 .. size[2]
	{
		let dz = z as f32 - center.z;
		for y in 0 .. size[1]
		{
			let dy = y as f32 - center.y;
			for x in 0 .. size[0]
			{
				let dx = x as f32 - center.x;
				let vec = Vec3f::new(dx * 0.7, dy * 0.7, dz);
				let vec_square_len = vec.magnitude2();
				let scale = 8.0 * ((-1.0 / 64.0) * vec_square_len).exp();

				let e = (z as f32) * frequency_mul_2pi;

				let electric_vector = Vec3f::new(scale * e.sin(), 0.0, 0.0);
				let magnetic_vector = Vec3f::unit_z().cross(electric_vector);

				let address = (x + y * size[0] + z * (size[0] * size[1])) as usize;
				electric_data[address] = electric_vector.extend(0.0).into();
				magnetic_data[address] = magnetic_vector.extend(0.0).into();
			}
		}
	}

	ElectromagneticField {
		electric_field: VectorField::new_with_data(display, size, &electric_data),
		magnetic_field: VectorField::new_with_data(display, size, &magnetic_data),
	}
}

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
