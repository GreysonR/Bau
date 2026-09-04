use bevy::prelude::*;

mod constraint_options;
pub use constraint_options::*;

mod spring;
pub use spring::Spring;


// mod fixed_distance;
// pub use fixed_distance::FixedDistance;

use super::rigid_body::*;

#[bevy_trait_query::queryable]
#[allow(unused)]
pub trait Constraint {
	fn solve_velocity(&mut self, bodies: &mut Query<RigidBodyQuery>, delta_time: f32) -> Result<(), BevyError> { Ok(()) }
	fn solve_position(&mut self, bodies: &mut Query<RigidBodyQuery>, delta_time: f32) -> Result<(), BevyError> { Ok(()) }
	fn post_update(&mut self, bodies: &mut Query<RigidBodyQuery>) -> Result<(), BevyError> { Ok(()) }
}