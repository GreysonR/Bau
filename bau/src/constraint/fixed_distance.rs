use bevy::prelude::*;
use std::f32::consts::PI;

use super::RigidBody;
use super::Constraint;

#[derive(Component, Debug)]
pub struct FixedDistance {
	pub body_a: Option<Entity>,
	pub body_a_offset: Vec2,

	pub body_b: Option<Entity>,
	pub body_b_offset: Vec2,

	pub length: f32,
}
impl Default for FixedDistance {
	fn default() -> Self {
		Self {
			body_a: None,
			body_a_offset: Vec2::ZERO,

			body_b: None,
			body_b_offset: Vec2::ZERO,

			length: 100.0,
		}
	}
}
