use bevy::{ prelude::*, window::WindowCloseRequested };
use bevy_prototype_lyon::prelude::*;

use bau::{ Body, Spring };

mod body_render;
use body_render::BodyBuilder;

mod spring_render;
use spring_render::SpringBuilder;


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
			.add_systems(Startup, (setup_render, add_bodies))
			// .add_systems(Update, print_time)
			.add_systems(Update, (body_render::update, spring_render::update))
			.add_systems(Update, handle_input)
			;
	}
}


// Keyboard input
fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut close_events: MessageWriter<WindowCloseRequested>, windows: Query<Entity, With<Window>>, bodies: Query<&mut Body>) {
	if keys.just_pressed(KeyCode::KeyQ) {
		let window = windows.single();
		if let Err(_) = window { return; }
		let window = window.unwrap();
		close_events.write(WindowCloseRequested { window });
	}
	if keys.any_just_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]) {
		let intent = Vec2::new(
			(keys.just_pressed(KeyCode::KeyD) as i32 - keys.just_pressed(KeyCode::KeyA) as i32) as f32,
			(keys.just_pressed(KeyCode::KeyW) as i32 - keys.just_pressed(KeyCode::KeyS) as i32) as f32,
		).normalize();

		for mut body in bodies {
			let impulse = 1000.0 * body.mass;
			body.velocity += impulse * intent;
			break; // only apply to 1st body
		}
	}
}


// Render init
fn setup_render(mut commands: Commands) {
	commands.spawn((Camera2d, Msaa::Sample4));
	commands.insert_resource(ClearColor(color_hex("#0C4440")));
}

fn add_bodies(mut commands: Commands) {
	// Add body
	let body = Body {
		position: Vec2::new(200.0, 0.0),
		velocity: Vec2::new(-40.0, 0.0),
		mass: 1.0,
		..Default::default()
	};
	let body_id = BodyBuilder::new(body)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);

	// Add spring constraint
	let spring = Spring {
		position: Vec2::new(0.0, 0.0),
		length: 100.0,
		stiffness: 150.0,
		damping: 2.0,
		body: body_id,
		..Default::default()
	};
	SpringBuilder::new(spring)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
}
