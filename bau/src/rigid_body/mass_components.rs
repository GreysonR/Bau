use bevy::prelude::*;

#[derive(Component)]
pub struct Mass {
	value: f32,
	inverse: f32,
}
impl Mass {
	pub fn new(mass: f32) -> Self {
		Self {
			value: mass,
			inverse: 1.0 / mass,
		}
	}
	pub fn infinite() -> Self {
		Self {
			value: f32::INFINITY,
			inverse: 0.0,
		}
	}
	pub fn set(&mut self, mass: f32) {
		self.value = mass;
		self.inverse = 1.0 / mass;
	}
	pub fn value(&self) -> f32 { self.value }
	pub fn inverse(&self) -> f32 { self.inverse }
}
impl Default for Mass {
	fn default() -> Self {
	    Self { value: 1.0, inverse: 1.0 }
	}
}


#[derive(Component)]
pub struct Inertia {
	value: f32,
	inverse: f32,
}
impl Inertia {
	pub fn new(inertia: f32) -> Self {
		Self {
			value: inertia,
			inverse: 1.0 / inertia,
		}
	}
	pub fn infinite() -> Self {
		Self {
			value: f32::INFINITY,
			inverse: 0.0,
		}
	}
	pub fn set(&mut self, inertia: f32) {
		self.value = inertia;
		self.inverse = 1.0 / inertia;
	}
	pub fn value(&self) -> f32 { self.value }
	pub fn inverse(&self) -> f32 { self.inverse }
}
impl Default for Inertia {
	fn default() -> Self {
	    Self { value: 1.0, inverse: 1.0 }
	}
}