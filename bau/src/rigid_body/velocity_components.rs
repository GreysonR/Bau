use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
impl Velocity {
	pub fn new(x: f32, y: f32) -> Self {
		Self(Vec2::new(x, y))
	}
}
impl Into<Vec2> for Velocity {
	fn into(self) -> Vec2 {
	    self.0
	}
}

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default)]
pub struct Angle(pub f32);
