use super::math_types::*;

// Returns params with simple depth test and write enabled.
pub fn get_default_drawing_params() -> glium::DrawParameters<'static>
{
	glium::DrawParameters {
		depth: glium::Depth {
			test: glium::DepthTest::IfLessOrEqual,
			write: true,
			range: (0.0, 1.0),
			clamp: glium::draw_parameters::DepthClamp::NoClamp,
		},
		..Default::default()
	}
}

pub fn make_uniform_matrix(mat: &Mat4f) -> [[f32; 4]; 4]
{
	Into::<[[f32; 4]; 4]>::into(mat.transpose())
}
