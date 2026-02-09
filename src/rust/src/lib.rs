pub type Geo = f32;
pub static TAU: Geo = core::f32::consts::TAU;
pub type Id = u16;
pub type PairId = u64;
pub type Time = Geo;
pub type Frame = u32;

mod debug;
use debug::log;


mod vec2;
pub use vec2::Vec2;

mod color;
pub use color::Color;

mod collision_pair;
pub use collision_pair::CollisionPair;

mod body;
pub use body::{Body, BodyOptions};

mod world;
pub use world::World;

mod physics;

mod engine;
pub use engine::{Engine, BodyMap};

mod bounds;
pub use bounds::Bounds;

mod grid;
pub use grid::Grid;
