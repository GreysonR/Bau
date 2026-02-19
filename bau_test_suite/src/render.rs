use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod body_render;
pub use body_render::BodyRenderBuilder;

mod spring_render;
pub use spring_render::SpringRenderBuilder;


// Useful render methods
pub fn color_hex(hex: &str) -> Color {
	Color::Srgba(Srgba::hex(hex).unwrap())
}


// Main plugin
pub struct Render;
impl Plugin for Render {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(ShapePlugin)
			.add_systems(Startup, init_render)
			.add_systems(Update, (body_render::update, spring_render::update))
			;
	}
}


// Creates basic resources required for 2d rendering
fn init_render(mut commands: Commands) {
	commands.spawn((Camera2d, Msaa::Sample4));
	commands.insert_resource(ClearColor(color_hex("#0C4440")));
}
