use super::keyboard_state;

pub struct CameraController {}

impl CameraController
{
	pub fn new() -> Self
	{
		Self {}
	}

	pub fn update(&mut self, time_delta_s: f32, keyboard_state: &keyboard_state::KeyboardState) {}
}
