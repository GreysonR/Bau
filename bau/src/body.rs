use bevy::prelude::*;

mod builder;
pub use builder::BodyBuilder;


#[derive(Component, Debug)]
pub struct Body {
	// Stateful properties
	pub vertices: Vec<Vec2>,
	pub position: Vec2,
	pub last_position: Vec2,
	pub angle: f32,
	pub last_angle: f32,
	
	// Inherent
	pub mass: f32,
	pub friction_air: f32,
	pub friction_angular: f32,
	pub is_static: bool,
	
	// Calculated from other properties
	pub inverse_mass: f32,
	pub inertia: f32, // calculated from vertices
	pub inverse_inertia: f32,
}

impl Default for Body {
	fn default() -> Self {
		Self {
			vertices: Vec::new(),
			position: Vec2::ZERO,
			last_position: Vec2::ZERO,
			angle: 0.0,
			last_angle: 0.0,
			
			mass: 1.0,
			inertia: 1.0,
			friction_air: 0.5,
			friction_angular: 0.1,
			is_static: false,

			inverse_mass: 1.0,
			inverse_inertia: 1.0,
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

	pub fn translate_position(&mut self, translation: Vec2) {
		for vertex in self.vertices.iter_mut() {
			*vertex += translation;
		}
		self.position += translation;
		self.last_position += translation;
	}
	pub fn translate_angle(&mut self, translation: f32) {
		let angle_vec = Vec2::from_angle(translation);
		for vertex in self.vertices.iter_mut() {
			*vertex = (*vertex - self.position).rotate(angle_vec) + self.position;
		}
		self.angle += translation;
		self.last_angle += translation;
	}
	pub fn set_position(&mut self, position: Vec2) {
		self.translate_position(position - self.position);
	}
	pub fn set_angle(&mut self, angle: f32) {
		self.translate_angle(angle - self.angle);
	}

	pub fn get_velocity(&self) -> Vec2 {
		self.position - self.last_position
	}
	pub fn set_velocity(&mut self, new_velocity: Vec2) {
		self.last_position = self.position - new_velocity;
	}
	pub fn get_angular_velocity(&self) -> f32 {
		self.angle - self.last_angle
	}
	pub fn set_angular_velocity(&mut self, new_angular_velocity: f32) {
		self.last_angle = self.angle - new_angular_velocity;
	}

	pub fn apply_impulse(&mut self, impulse: Vec2, position: Vec2) {
		let radius = position - self.position;
		let cross = radius.perp_dot(impulse);

		self.set_velocity(self.get_velocity() + impulse * self.inverse_mass);
		self.set_angular_velocity(self.get_angular_velocity() + cross * self.inverse_inertia);
	}

	pub fn contains_point(&self, point: Vec2) -> bool {
		let vertices = &self.vertices;
		let len = vertices.len();
		for i in 0..len {
			let j = (i + 1) % len;
			let va = vertices[i];
			let vb = vertices[j];
			let axis = vb - va;

			let dist = axis.perp_dot(point - va);
			if dist < 0.0 {
				return false;
			}
		}
		true
	}

	// Finds the velocity of the given point on the body, taking into account both linear and angular velocity
	pub fn get_velocity_at_point(&self, point: Vec2) -> Vec2 {
		self.get_velocity() + self.get_angular_velocity() * (point - self.position).perp()
	}
}
