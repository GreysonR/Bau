use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);