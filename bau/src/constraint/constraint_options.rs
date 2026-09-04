use bevy::prelude::*;

#[derive(Component)]
pub struct BodyA(pub Entity, pub Vec2);
impl Default for BodyA {
	fn default() -> Self { Self(Entity::PLACEHOLDER, Vec2::ZERO) }
}

#[derive(Component)]
pub struct BodyB(pub Entity, pub Vec2);
impl Default for BodyB {
	fn default() -> Self { Self(Entity::PLACEHOLDER, Vec2::ZERO) }
}

#[derive(Component, Default)]
pub struct UnstretchedLength(pub f32);