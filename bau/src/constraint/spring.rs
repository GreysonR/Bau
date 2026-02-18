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
	fn solve(&self, bodies: &mut Query<&mut Body>) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let ds = self.position - body.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let rel_vel = body.velocity.dot(dir);

		let mut impulse = (ds.length() - self.length) * self.stiffness;
		impulse -= self.damping * rel_vel;

		body.accumulated_impulse += impulse * dir;
	}
}