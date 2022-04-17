pub struct Character
{
	name: String,
	level: i32
}

impl Character
{
	pub fn new(name: String, level: i32) -> Self
	{
		Self
		{
			name: name,
			level: level
		}
	}

	pub fn greeting(&self)
	{
		println!("Hello, My name is {}.", self.name);
	}
}