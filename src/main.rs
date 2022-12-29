mod camera_controller;
mod keyboard_state;
#[allow(dead_code)]
mod math_types;

use glium::glutin;
use math_types::*;

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

	let fernweh = Fernweh::new(&display);

	let mut prev_time = std::time::Instant::now();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = match event
		{
			glutin::event::Event::WindowEvent { event, .. } => match event
			{
				// Break from the main loop when the window is closed.
				glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				_ => glutin::event_loop::ControlFlow::Poll,
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
					glutin::event_loop::ControlFlow::Poll
				},
			},
			glutin::event::Event::MainEventsCleared =>
			{
				let cur_time = std::time::Instant::now();
				let time_delta_s = (cur_time - prev_time).as_secs_f32().max(0.00001).min(0.1);
				prev_time = cur_time;

				camera_controller.update(time_delta_s, &keyboard_state);

				let mut target = display.draw();
				fernweh.draw(&mut target);
				target.finish().unwrap();
				glutin::event_loop::ControlFlow::Poll
			},
			_ => glutin::event_loop::ControlFlow::Poll,
		};
	});
}

struct Fernweh
{
	vertex_buffer: glium::VertexBuffer<Vertex>,
	index_buffer: glium::IndexBuffer<u16>,
	program: glium::Program,
	start_time: std::time::Instant,
}

impl Fernweh
{
	fn new(display: &glium::Display) -> Self
	{
		let vertices = [
			Vertex {
				position: [-0.5, -0.5, -2.0],
				color: [0.0, 1.0, 0.0, 1.0],
			},
			Vertex {
				position: [0.0, 0.5, -2.0],
				color: [0.0, 0.0, 1.0, 0.5],
			},
			Vertex {
				position: [0.5, -0.5, -1.5],
				color: [1.0, 0.0, 0.0, 0.25],
			},
			Vertex {
				position: [0.0, 0.0, -2.0],
				color: [1.0, 1.0, 1.0, 0.125],
			},
		];

		let indices = [0, 1, 2, 0, 2, 3];

		let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
		let index_buffer =
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

		let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

		Self {
			vertex_buffer,
			index_buffer,
			program,
			start_time: std::time::Instant::now(),
		}
	}

	fn draw<S: glium::Surface>(&self, surface: &mut S)
	{
		let cur_time = std::time::Instant::now();
		let abs_time_s = (cur_time - self.start_time).as_secs_f32();

		surface.clear_color(0.0, 0.0, 0.0, 0.0);

		let (width, height) = surface.get_dimensions();
		let aspect = (width as f32) / (height as f32);

		let rotation_matrix = Mat4f::from_angle_z(Rad(abs_time_s));
		let perspective_matrix = cgmath::perspective(Rad(1.5), aspect, 0.1, 128.0);
		let matrix = perspective_matrix * rotation_matrix;

		let uniforms = glium::uniform! {
			matrix: Into::<[[f32; 4];4]>::into(matrix.transpose())
		};

		let drawing_params = glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			..Default::default()
		};

		surface
			.draw(
				&self.vertex_buffer,
				&self.index_buffer,
				&self.program,
				&uniforms,
				&drawing_params,
			)
			.unwrap();
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
