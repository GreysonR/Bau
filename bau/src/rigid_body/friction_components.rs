use bevy::prelude::*;

#[derive(Component)]
pub struct Friction(pub f32);
impl Default for Friction {
	fn default() -> Self { Self(0.1) }
}

#[derive(Component)]
pub struct FrictionAir(pub f32);
impl Default for FrictionAir {
	fn default() -> Self { Self(0.5) }
}

#[derive(Component)]
pub struct FrictionAngular(pub f32);
impl Default for FrictionAngular {
	fn default() -> Self { Self(0.1) }
}