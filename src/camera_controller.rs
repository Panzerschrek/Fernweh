use super::{keyboard_state, math_types::*};

use glium::glutin::event::VirtualKeyCode;

pub struct CameraController
{
	azimuth: RadiansF,
	elevation: RadiansF,
	pos: Vec3f,
}

impl CameraController
{
	pub fn new() -> Self
	{
		Self {
			azimuth: RadiansF::zero(),
			elevation: RadiansF::zero(),
			pos: Vec3f::zero(),
		}
	}

	pub fn update(&mut self, time_delta_s: f32, keyboard_state: &keyboard_state::KeyboardState)
	{
		const ANGLE_SPEED: RadiansF = Rad(2.0);

		const PI: RadiansF = Rad(std::f32::consts::PI);
		let half_pi = PI / 2.0;
		let two_pi = PI * 2.0;

		if keyboard_state.is_pressed(&VirtualKeyCode::Left)
		{
			self.azimuth += ANGLE_SPEED * time_delta_s;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::Right)
		{
			self.azimuth -= ANGLE_SPEED * time_delta_s;
		}

		if keyboard_state.is_pressed(&VirtualKeyCode::Up)
		{
			self.elevation += ANGLE_SPEED * time_delta_s;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::Down)
		{
			self.elevation -= ANGLE_SPEED * time_delta_s;
		}

		while self.azimuth > PI
		{
			self.azimuth -= two_pi;
		}
		while self.azimuth < -PI
		{
			self.azimuth += two_pi;
		}

		if self.elevation > half_pi
		{
			self.elevation = half_pi;
		}
		if self.elevation < -half_pi
		{
			self.elevation = -half_pi;
		}

		let rotate = self.get_rotation();

		let forward_vector = rotate.rotate_vector(Vec3f::unit_x());
		let right_vector = rotate.rotate_vector(Vec3f::unit_y());
		let mut move_vector = Vec3f::zero();

		const SPEED: f32 = 16.0;
		const JUMP_SPEED: f32 = 0.8 * SPEED;

		if keyboard_state.is_pressed(&VirtualKeyCode::W)
		{
			move_vector += forward_vector;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::S)
		{
			move_vector -= forward_vector;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::D)
		{
			move_vector -= right_vector;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::A)
		{
			move_vector += right_vector;
		}

		let move_vector_length = move_vector.magnitude();
		if move_vector_length > 0.0
		{
			self.pos += move_vector * (time_delta_s * SPEED / move_vector_length);
		}

		if keyboard_state.is_pressed(&VirtualKeyCode::Space)
		{
			self.pos.z += time_delta_s * JUMP_SPEED;
		}
		if keyboard_state.is_pressed(&VirtualKeyCode::C)
		{
			self.pos.z -= time_delta_s * JUMP_SPEED;
		}
	}

	pub fn get_view_matrix(&self, aspect: f32) -> Mat4f
	{
		let mut basis_change = Mat4f::identity();
		basis_change.x.x = 0.0;
		basis_change.y.y = 0.0;
		basis_change.z.z = 0.0;
		basis_change.x.z = -1.0;
		basis_change.y.x = -1.0;
		basis_change.z.y = 1.0;

		let perspective = cgmath::perspective(Rad(std::f32::consts::PI) * 0.5, aspect, 0.1, 1024.0);
		let rotation = Mat4f::from(self.get_rotation().conjugate());
		let translation = Mat4f::from_translation(-self.pos);

		perspective * basis_change * rotation * translation
	}

	fn get_rotation(&self) -> QuaternionF
	{
		let rotate_y = QuaternionF::from_angle_y(-self.elevation);
		let rotate_z = QuaternionF::from_angle_z(self.azimuth);
		rotate_z * rotate_y
	}
}
