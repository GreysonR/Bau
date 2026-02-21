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
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, delta_time: f32);
	fn solve_position(&self, _bodies: &mut Query<&mut Body>, _delta_time: f32) {}
}
