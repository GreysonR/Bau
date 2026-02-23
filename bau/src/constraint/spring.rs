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
	pub position_offset: Vec2,
}
impl Default for Spring {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,
			length: 100.0,
			stiffness: 100.0,
			damping: 2.0,
			position_offset: Vec2::ZERO,
		}
	}
}

impl ConstraintSolver for Spring { // TODO: make this work with multiple constraint iterations
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, _delta_time: f32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let body_position = body.position + radius;
		let ds = body_position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		
		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);

		let mut impulse = (self.length - ds.length()).min(0.0) * self.stiffness;
		impulse -= self.damping * rel_vel.min(0.0);

		let p = impulse * dir;
		body.apply_impulse(p, body_position);
	}
}