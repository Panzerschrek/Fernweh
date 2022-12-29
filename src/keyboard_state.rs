use std::collections::HashMap;

use glium::glutin::event::{ElementState, VirtualKeyCode};

pub struct KeyboardState(HashMap<VirtualKeyCode, ElementState>);

impl KeyboardState
{
	pub fn new() -> Self
	{
		Self(HashMap::new())
	}

	pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool
	{
		self.0.get(key).map(|&s| s == ElementState::Pressed).unwrap_or(false)
	}

	pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode)
	{
		match key_state
		{
			ElementState::Pressed =>
			{
				self.0.insert(code, key_state);
			},
			ElementState::Released =>
			{
				self.0.remove(&code);
			},
		}
	}
}
