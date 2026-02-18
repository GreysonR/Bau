use bevy::{ prelude::*, window::WindowCloseRequested };
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
		.add_systems(Update, (move_spring, handle_input))
		.run();

}

#[derive(Resource)]
struct MainSpring(Entity);

fn add_bodies(mut commands: Commands) {
	// Add bodies
	let body_a = Body {
		position: Vec2::new(200.0, 0.0),
		velocity: Vec2::new(-40.0, 0.0),
		mass: 1.0,
		..Default::default()
	};
	let body_a_id = BodyRenderBuilder::new(body_a)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);
	
	let body_b = Body {
		position: Vec2::new(-500.0, -300.0),
		velocity: Vec2::new(800.0, 1000.0),
		mass: 1.0,
		..Default::default()
	};
	let _body_b_id = BodyRenderBuilder::new(body_b)
		.fill(color_hex("#E35531"))
		.build(&mut commands);

	// Add spring constraint
	let spring = Spring {
		position: Vec2::new(0.0, 0.0),
		length: 100.0,
		stiffness: 150.0,
		damping: 2.0,
		body: body_a_id,
		..Default::default()
	};
	let spring = SpringRenderBuilder::new(spring)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);

	commands.insert_resource(MainSpring(spring));
}


// Mouse input
fn move_spring(mouse_buttons: Res<ButtonInput<MouseButton>>, spring_id: Res<MainSpring>, camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, window: Single<&Window, With<PrimaryWindow>>, mut springs: Query<&mut Constraint>) {
	if let Some(position) = window.cursor_position() && mouse_buttons.pressed(MouseButton::Left) {
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


// Keyboard input
fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut close_events: MessageWriter<WindowCloseRequested>, windows: Query<Entity, With<Window>>, bodies: Query<&mut Body>) {
	if keys.just_pressed(KeyCode::KeyQ) {
		let window = windows.single();
		if let Err(_) = window { return; }
		let window = window.unwrap();
		close_events.write(WindowCloseRequested { window });
	}
	if keys.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]) {
		let intent = Vec2::new(
			(keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32,
			(keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32) as f32,
		).normalize();

		for mut body in bodies {
			let impulse = 100.0 * body.mass;
			body.velocity += impulse * intent;
			break; // only apply to 1st body
		}
	}
}