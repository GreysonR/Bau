use bevy::prelude::*;
use super::body::Body;

mod spring;
pub use spring::Spring;

#[derive(Component)]
pub enum Constraint {
	Spring(Spring),
}

pub trait ConstraintSolver {
	fn solve(&self, bodies: &mut Query<&mut Body>);
}
