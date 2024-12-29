extern crate web_sys;

type Geo = f32;
type Id = u64;

mod vec2;
pub use vec2::Vec2;

mod color;
pub use color::Color;


mod body;
pub use body::Body;

mod world;
pub use world::World;

mod physics;
pub use physics::Physics;

mod engine;
pub use engine::Engine;
