use bevy::prelude::*;

use super::Body;
use super::ConstraintSolver;

#[derive(Component, Debug)]
pub struct FixedDistance {
	pub body: Entity,
	pub position: Vec2,
	pub length: f32,
}
impl Default for FixedDistance {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,
			length: 100.0,
		}
	}
}

impl ConstraintSolver for FixedDistance {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, _delta_time: f32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let ds = body.position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let rel_vel = body.velocity.dot(dir);

		let impulse = -rel_vel; // velocity in dir should go to 0; i.e. impulse + rel_vel = 0; so impulse = -rel_vel

		// sort of baumgarte stabilization like
		// let c = ds.length() - self.length;
		// let k = 100.0;
		// let delta_impulse = -c * k;
		// impulse += delta_impulse;

		body.velocity += impulse * dir;
	}
	fn solve_position(&self, bodies: &mut Query<&mut Body>, delta_time: f32) {
		// return;
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let ds = body.position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		
		/*
			C = 0			definition of general constraint
			C = |s| - L,	definition of fixed distance constraint
			|s| = L; 		plugging in C = 0

			dir = s / |s|	definition of dir
			|s| = s / dir;	solve for |s|
			
			s / dir = L;	plug in |s| = s / dir
			[ s = L * dir ]	soln
		*/
		let target = self.length * dir + self.position;
		let diff = target - body.position;
		let position_stiffness: f32 = 1.0e-2;
		body.position += diff * position_stiffness.powf(delta_time * 10.0); // * 10.0 so position stiffness doesn't have to be absurdely low
	}
}
