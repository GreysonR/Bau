use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bau::{ Body, Spring, Constraint };

mod render;
use render::{ color_hex, BodyRenderBuilder, SpringRenderBuilder };

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// .add_systems(Update, print_mouse_position)
		.add_plugins((bau::Engine::default(), render::Render))
		.add_systems(Startup, add_bodies)
		.add_systems(Update, move_spring)
		.run();

}

#[derive(Resource)]
struct MainSpring(Entity);

fn add_bodies(mut commands: Commands) {
	// Add body
	let body = Body {
		position: Vec2::new(200.0, 0.0),
		velocity: Vec2::new(-40.0, 0.0),
		mass: 1.0,
		..Default::default()
	};
	let body_id = BodyRenderBuilder::new(body)
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
	let spring = SpringRenderBuilder::new(spring)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);

	commands.insert_resource(MainSpring(spring));
}


fn move_spring(spring_id: Res<MainSpring>, camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, window: Single<&Window, With<PrimaryWindow>>, mut springs: Query<&mut Constraint>) {
	if let Some(position) = window.cursor_position() {
		let mut constraint = springs.get_mut(spring_id.0).expect("spring should be in world");
		match constraint.as_mut() {
			Constraint::Spring(spring) => {
				let (camera, camera_transform) = camera.single().expect("camera should be in world");
				let world_pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();
				spring.position.x = world_pos.x;
				spring.position.y = world_pos.y;
			}
		}
		// println!("mouse is at ({}, {})", position.x, position.y);
	}
	// else not in window
}
