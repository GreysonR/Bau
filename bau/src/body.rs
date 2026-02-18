use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Body {
	// Stateful properties
	pub vertices: Vec<Vec2>,
	pub position: Vec2,
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
			position: Vec2::ZERO,
			velocity: Vec2::ZERO,
			vertices: Vec::new(),
			
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

#[allow(unused)] // TODO: maybe remove these allows
pub struct BodyBuilder { // hell yeah
	// Stateful properties
	position: Vec2,
	velocity: Vec2,
	vertices: Vec<Vec2>,
	
	// Inherent
	mass: f32,
	friction_air: f32,
	is_static: bool,
}
impl Default for BodyBuilder {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			velocity: Vec2::ZERO,
			vertices: Vec::new(),

			mass: 1.0,
			friction_air: 1.0,
			is_static: false,
		}
	}
}

#[allow(unused)]
impl BodyBuilder {
	pub fn from_vertices(vertices: Vec<Vec2>) -> Self {
		// TODO: assert!(vertices.length() >= 3, "Body must have at least 3 vertices");
		Self {
			vertices,
			..Default::default()
		}
	}
	pub fn position(mut self, position: Vec2) -> Self { self.position = position; self }
	pub fn velocity(mut self, velocity: Vec2) -> Self { self.velocity = velocity; self }
	pub fn mass(mut self, mass: f32) -> Self { self.mass = mass; self }
	pub fn friction_air(mut self, friction_air: f32) -> Self { self.friction_air = friction_air; self }
	pub fn is_static(mut self, is_static: bool) -> Self { self.is_static = is_static; self }

	fn get_inertia(&self) -> f32 {
		1.0 // TODO: calculate this correctly from vertices
	}
	pub fn build(self) -> Body {
		let is_static = self.is_static;
		let inertia = self.get_inertia();
		let inverse_inertia = if is_static { 0.0 } else { 1.0 / inertia };
		let inverse_mass = if is_static { 0.0 } else { 1.0 / self.mass };

		Body {
			vertices: self.vertices,
			position: self.position,
			velocity: self.velocity,

			mass: self.mass,
			friction_air: self.friction_air,
			is_static,

			inverse_mass,
			inertia,
			inverse_inertia,

			..Default::default() // don't init any solver properties
		}
	}
}
