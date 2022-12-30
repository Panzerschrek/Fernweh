pub struct VectorField
{
	size: [u32; 3],
	buffer: BufferType,
}

// Use 4-float vector instead of 3-float vector because of packing issues in shaders.
type BufferType = glium::buffer::Buffer<[[f32; 4]]>;

impl VectorField
{
	pub fn new(display: &glium::Display, size: [u32; 3]) -> Self
	{
		let data = vec![[0.0; 4]; (size[0] * size[1] * size[2]) as usize];
		Self::new_with_data(display, size, &data)
	}

	pub fn new_with_data(display: &glium::Display, size: [u32; 3], data: &[[f32; 4]]) -> Self
	{
		assert_eq!(data.len(), (size[0] * size[1] * size[2]) as usize);

		Self {
			buffer: glium::buffer::Buffer::new(
				display,
				data,
				glium::buffer::BufferType::ShaderStorageBuffer,
				glium::buffer::BufferMode::Default,
			)
			.unwrap(),
			size,
		}
	}

	pub fn get_size(&self) -> [u32; 3]
	{
		self.size
	}

	pub fn get_buffer(&self) -> &BufferType
	{
		&self.buffer
	}

	pub fn get_buffer_mut(&mut self) -> &mut BufferType
	{
		&mut self.buffer
	}
}
