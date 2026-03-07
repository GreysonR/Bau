use bevy::{ prelude::*, window::WindowCloseRequested };
use bevy::window::PrimaryWindow;

use bau::{ Body, BodyBuilder, Constraint, FixedDistance, Spring };

mod render;
use render::{ color_hex, BodyRenderBuilder, SpringRenderBuilder, DistanceRenderBuilder };

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
struct MouseBody(Entity);

fn add_bodies(mut commands: Commands) {
	// Add bodies
	let body_a = BodyBuilder::rect(50.0, 50.0)
		.position(Vec2::new(200.0, 0.0))
		.velocity(Vec2::new(-40.0, 0.0))
		.angle(std::f32::consts::PI * 0.25)
		// .mass(1.0)
		.build();
	let body_a_id = BodyRenderBuilder::new(body_a)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);
	

	let body_b = BodyBuilder::circle(3.0)
		.position(Vec2::new(0.0, 0.0))
		.is_static(true)
		.build();
	let body_b_id = BodyRenderBuilder::new(body_b)
		.fill(color_hex("#E35531"))
		.build(&mut commands);
	commands.insert_resource(MouseBody(body_b_id));
	
	let body_c = BodyBuilder::rect(30.0, 30.0)
		.position(Vec2::new(200.0, 0.0))
		.velocity(Vec2::new(-40.0, 0.0))
		// .mass(150.0)
		.build();
	let body_c_id = BodyRenderBuilder::new(body_c)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);

	
	// Add spring constraints
	let spring = Spring {
		body_a: body_a_id,
		body_a_offset: Vec2::new(25.0, 25.0),
		body_b: body_b_id,

		length: 100.0,
		frequency: 2.0,
		damping: 0.01,

		..Default::default()
	};
	let _spring = SpringRenderBuilder::new(spring)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
	
	let spring2 = Spring {
		body_a: body_a_id,
		body_a_offset: Vec2::new(-25.0, 25.0),
		body_b: body_c_id,
		body_b_offset: Vec2::new(0.0, 15.0),

		length: 50.0,
		frequency: 1.0,
		damping: 0.01,

		..Default::default()
	};
	let _spring2 = SpringRenderBuilder::new(spring2)
		.stroke((color_hex("#fdf2d9b2"), 2.0))
		.build(&mut commands);


	// Add fixed distance constraint
	let fixed_dist = FixedDistance {
		position: Vec2::new(100.0, 0.0),
		length: 100.0,
		body: body_a_id,
		position_offset: Vec2::new(-25.0, -25.0),
		..Default::default()
	};
	let _fixed_dist = DistanceRenderBuilder::new(fixed_dist)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
}


// Mouse input
fn move_spring(mouse_buttons: Res<ButtonInput<MouseButton>>, body_id: Res<MouseBody>, camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, window: Single<&Window, With<PrimaryWindow>>, mut bodies: Query<&mut Body>) {
	// Moving main spring constraint by clicking on window
	if window.cursor_position().is_none() || !mouse_buttons.pressed(MouseButton::Left) {
		return; // cursor not in window or not clicking
	}
	let position = window.cursor_position().unwrap(); // guaranteed successful unwrap

	let body = bodies.get_mut(body_id.0);
	if body.is_err() {
		return; // Don't do anything if constraint isn't in world
	}

	let mut body = body.unwrap();
	let (camera, camera_transform) = camera.single().expect("camera should be in world");
	let world_pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();
	body.set_position(world_pos);
	body.velocity = Vec2::ZERO;
}


// Keyboard input
fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut close_events: MessageWriter<WindowCloseRequested>, windows: Query<Entity, With<Window>>, bodies: Query<&mut Body>) {
	// Quick exiting window with q
	if keys.just_pressed(KeyCode::KeyQ) {
		let window = windows.single();
		if let Err(_) = window { return; }
		let window = window.unwrap();
		close_events.write(WindowCloseRequested { window });
	}

	// Applying force to 1st body with WASD
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