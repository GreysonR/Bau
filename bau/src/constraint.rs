use bevy::prelude::*;
use super::body::Body;

mod spring;
pub use spring::Spring;

mod fixed_distance;
pub use fixed_distance::FixedDistance;

#[derive(Component)]
pub enum Constraint {
	Spring(Spring),
	FixedDistance(FixedDistance),
}

pub trait ConstraintSolver {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, delta_time: f32, iterations: i32);
	fn solve_position(&self, _bodies: &mut Query<&mut Body>, _delta_time: f32, _iterations: i32) {}
}

impl ConstraintSolver for Constraint {
	fn solve_velocity(&self, mut bodies: &mut Query<&mut Body>, delta: f32, iterations: i32) {
		match self {
			Constraint::Spring(spring) => spring.solve_velocity(&mut bodies, delta, iterations),
			Constraint::FixedDistance(constraint) => constraint.solve_velocity(&mut bodies, delta, iterations),
		};
	}
	fn solve_position(&self, mut bodies: &mut Query<&mut Body>, delta: f32, iterations: i32) {
		match self {
			Constraint::Spring(spring) => spring.solve_position(&mut bodies, delta, iterations),
			Constraint::FixedDistance(constraint) => constraint.solve_position(&mut bodies, delta, iterations),
		};
	}
}