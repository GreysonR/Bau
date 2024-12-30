extern crate web_sys;

type Geo = f32;
type Id = u16;
type PairId = u64;
type Time = f32;
type Frame = u32;

mod vec2;
pub use vec2::Vec2;

mod color;
pub use color::Color;

mod collision_pair;
pub use collision_pair::CollisionPair;

mod body;
pub use body::Body;

mod world;
pub use world::World;

mod physics;

mod engine;
pub use engine::Engine;
