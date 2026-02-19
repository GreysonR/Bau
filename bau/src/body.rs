use bevy::prelude::*;

mod builder;
pub use builder::BodyBuilder;


#[derive(Component, Debug)]
pub struct Body {
	// Stateful properties
	pub vertices: Vec<Vec2>,
	pub position: Vec2,
	pub angle: f32,
	pub velocity: Vec2,
	
	// Inherent
	pub mass: f32,
	pub friction_air: f32,
	pub is_static: bool,
	
	// Calculated from other properties
	pub inverse_mass: f32,
	pub inertia: f32, // calculated from vertices
	pub inverse_inertia: f32,

	// Solved
	pub accumulated_impulse: Vec2,
}

impl Default for Body {
	fn default() -> Self {
		Self {
			vertices: Vec::new(),
			position: Vec2::ZERO,
			angle: 0.0,
			velocity: Vec2::ZERO,
			
			mass: 1.0,
			inertia: 1.0,
			friction_air: 0.5,
			is_static: false,

			inverse_mass: 1.0,
			inverse_inertia: 1.0,

			accumulated_impulse: Vec2::ZERO,
		}
	}
}

impl Body {
	/*
	- TODO: cache transformed vertices; enable dirty flag when position or angle is set; re-transform vertices next time you get them
	- For now, I'm not going to worry about using getter/setters as they're a bit of a pain for little gain
	- If needed I can make the fields private and the compiler will tell me everything I need to update

	// Getters
	pub fn get_vertices(&self) -> &Vec<Vec2> { &self.vertices }
	pub fn get_position(&self) -> &Vec2 { &self.position }
	pub fn get_angle(&self) -> f32 { self.angle }
	pub fn get_velocity(&self) -> &Vec2 { &self.velocity }
	// Setters
	pub fn set_position(&mut self, position: &Vec2) { self.position.x = position.x; self.position.y = position.y; }
	pub fn set_angle(&mut self, angle: f32) { self.angle = angle; }
	pub fn set_velocity(&mut self, velocity: &Vec2) { self.velocity.x = velocity.x; self.velocity.y = velocity.y; }
	*/

	
}
