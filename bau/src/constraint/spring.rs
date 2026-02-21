use bevy::prelude::*;

use super::Body;
use super::ConstraintSolver;

#[derive(Component, Debug)]
pub struct Spring {
	pub body: Entity,
	pub position: Vec2,
	pub length: f32,
	pub stiffness: f32,
	pub damping: f32,
}
impl Default for Spring {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,
			length: 100.0,
			stiffness: 100.0,
			damping: 2.0,
		}
	}
}

impl ConstraintSolver for Spring {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, _delta_time: f32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let ds = body.position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let rel_vel = body.velocity.dot(dir);

		let mut impulse = (self.length - ds.length()) * self.stiffness;
		impulse -= self.damping * rel_vel;

		let p = impulse * body.inverse_mass;
		body.velocity += p * dir;
	}
}