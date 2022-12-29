#[allow(dead_code)]
mod math_types;

use glium::{glutin, Surface};
use math_types::*;

fn main()
{
	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new()
		.with_gl_profile(glutin::GlProfile::Core)
		.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3)))
		.with_vsync(true);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let vertices = [
		Vertex {
			position: [-0.5, -0.5],
			color: [0.0, 1.0, 0.0, 1.0],
		},
		Vertex {
			position: [0.0, 0.5],
			color: [0.0, 0.0, 1.0, 0.5],
		},
		Vertex {
			position: [0.5, -0.5],
			color: [1.0, 0.0, 0.0, 0.25],
		},
		Vertex {
			position: [0.0, 0.0],
			color: [1.0, 1.0, 1.0, 0.125],
		},
	];

	let indices = [0u16, 1, 2, 0, 2, 3];

	let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
	let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

	let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

	let start_time = std::time::Instant::now();

	let draw = move || {
		let cur_time = std::time::Instant::now();
		let abs_time_s = (cur_time - start_time).as_secs_f32();

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);

		let (width, height) = target.get_dimensions();
		let aspect = (height as f32) / (width as f32);

		let rotation_matrix = Mat4f::from_angle_z(Rad(abs_time_s));
		let aspect_matrix = Mat4f::from_nonuniform_scale(aspect, 1.0, 1.0);
		let matrix = rotation_matrix * aspect_matrix;

		let uniforms = glium::uniform! {
			matrix: Into::<[[f32; 4];4]>::into(matrix)
		};

		let drawing_params = glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			..Default::default()
		};

		target
			.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &drawing_params)
			.unwrap();
		target.finish().unwrap();
	};

	event_loop.run(move |event, _, control_flow| {
		*control_flow = match event
		{
			glutin::event::Event::WindowEvent { event, .. } => match event
			{
				// Break from the main loop when the window is closed.
				glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
				_ => glutin::event_loop::ControlFlow::Poll,
			},
			glutin::event::Event::MainEventsCleared =>
			{
				draw();
				glutin::event_loop::ControlFlow::Poll
			},
			_ => glutin::event_loop::ControlFlow::Poll,
		};
	});
}

#[derive(Copy, Clone)]
struct Vertex
{
	position: [f32; 2],
	color: [f32; 4],
}

glium::implement_vertex!(Vertex, position, color);

const VERTEX_SHADER: &str = r#"
	#version 430
	uniform mat4 matrix;
	in vec2 position;
	in vec4 color;
	out vec4 vColor;
	void main() {
		gl_Position = vec4(position, 0.0, 1.0) * matrix;
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
