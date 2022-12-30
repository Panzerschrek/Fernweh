mod camera_controller;
mod keyboard_state;
#[allow(dead_code)]
mod math_types;
mod ogl_common;
mod vector_field;
mod vector_field_visualizer;
mod fields_simulator;

use glium::{glutin, Surface};

fn main()
{
	let wb = glutin::window::WindowBuilder::new()
		.with_inner_size(glutin::dpi::PhysicalSize {
			width: 640,
			height: 480,
		})
		.with_min_inner_size(glutin::dpi::PhysicalSize {
			width: 320,
			height: 240,
		});

	let cb = glutin::ContextBuilder::new()
		.with_gl_profile(glutin::GlProfile::Core)
		.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3)))
		.with_vsync(true);

	let event_loop = glutin::event_loop::EventLoop::new();
	let mut keyboard_state = keyboard_state::KeyboardState::new();
	let mut camera_controller = camera_controller::CameraController::new();

	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let fields_simulator = fields_simulator::FieldsSimulator::new(&display);

	let mut prev_time = std::time::Instant::now();

	event_loop.run(move |event, _, control_flow| {
		match event
		{
			glutin::event::Event::WindowEvent { event, .. } => match event
			{
				// Break from the main loop when the window is closed.
				glutin::event::WindowEvent::CloseRequested =>
				{
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return;
				},
				glutin::event::WindowEvent::KeyboardInput {
					input: glutin::event::KeyboardInput {
						state, virtual_keycode, ..
					},
					..
				} =>
				{
					if let Some(code) = virtual_keycode
					{
						keyboard_state.process_event(state, code);
					}
				},
				_ =>
				{},
			},
			glutin::event::Event::MainEventsCleared =>
			{
				let cur_time = std::time::Instant::now();
				let time_delta_s = (cur_time - prev_time).as_secs_f32().max(0.00001).min(0.1);
				prev_time = cur_time;

				camera_controller.update(time_delta_s, &keyboard_state);

				let mut surface = display.draw();

				let (width, height) = surface.get_dimensions();
				let aspect = (width as f32) / (height as f32);
				let view_matrix = camera_controller.get_view_matrix(aspect);

				fields_simulator.draw(&mut surface, &view_matrix);

				surface.finish().unwrap();
			},
			_ =>
			{},
		};

		*control_flow = glutin::event_loop::ControlFlow::Poll;
	});
}
