use bevy::prelude::*;

#[derive(Component)]
pub struct Frequency(pub f32);
impl Default for Frequency {
	fn default() -> Self { Self(2.0) }
}

#[derive(Component)]
pub struct Damping(pub f32);
impl Default for Damping {
	fn default() -> Self { Self(0.5) }
}

#[derive(Component)]
pub struct AllowCompression(pub bool);
impl Default for AllowCompression {
	fn default() -> Self { Self(true) }
}