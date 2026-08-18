use bevy::prelude::*;

mod constraint_options;
pub use constraint_options::*;

mod spring;
pub use spring::Spring;


// mod fixed_distance;
// pub use fixed_distance::FixedDistance;

use crate::RigidBody;

#[bevy_trait_query::queryable]
#[allow(unused)]
pub trait Constraint {
	fn solve_velocity(&self, bodies: &mut Query<&mut RigidBody>, delta_time: f32, iterations: i32) -> Result<(), BevyError> { Ok(()) }
	fn solve_position(&self, bodies: &mut Query<&mut RigidBody>, delta_time: f32, iterations: i32) -> Result<(), BevyError> { Ok(()) }
}