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
impl Body {
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
