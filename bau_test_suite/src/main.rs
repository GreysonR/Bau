use bevy::{ prelude::*, window::WindowCloseRequested };
use bevy::window::PrimaryWindow;

use bau::{ Body, BodyBuilder, FixedDistance, Spring };

mod render;
use render::{ color_hex, BodyRenderBuilder, SpringRenderBuilder, DistanceRenderBuilder };

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// .add_systems(Update, print_mouse_position)
		.add_plugins((bau::Engine::default(), render::Render))
		.add_systems(Startup, add_bodies)
		.add_systems(Update, (handle_mouse, handle_input))
		.run();

}


fn add_bodies(mut commands: Commands) {
	commands.insert_resource(MouseBody(None));
	// Add bodies
	let body_a = BodyBuilder::rect(50.0, 50.0)
		.position(Vec2::new(200.0, 0.0))
		// .velocity(Vec2::new(-40.0, 0.0))
		.angle(std::f32::consts::PI * 0.2)
		// .mass(1.0)
		.build();
	let body_a_id = BodyRenderBuilder::new(body_a)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);
	
	
	let body_b = BodyBuilder::circle(10.0)
		.position(Vec2::new(0.0, 0.0))
		.is_static(true)
		.build();
	let body_b_id = BodyRenderBuilder::new(body_b)
		.fill(color_hex("#E35531"))
		.build(&mut commands);
	
	
	let body_c = BodyBuilder::rect(30.0, 30.0)
		.position(Vec2::new(200.0, 0.0))
		.velocity(Vec2::new(-40.0, 0.0))
		// .mass(150.0)
		.build();
	let body_c_id = BodyRenderBuilder::new(body_c)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);
	

	let pin = BodyBuilder::circle(3.0)
		.position(Vec2::new(100.0, 0.0))
		.is_static(true)
		.build();
	let pin_id = BodyRenderBuilder::new(pin)
		.fill(color_hex("#c1c1c168"))
		.build(&mut commands);


	let body_d = BodyBuilder::rect(50.0, 50.0)
		.position(Vec2::new(200.0, 400.0))
		// .velocity(Vec2::new(-40.0, 0.0))
		.angle(std::f32::consts::PI * 0.1)
		// .mass(1.0)
		.build();
	let body_d_id = BodyRenderBuilder::new(body_d)
		.fill(color_hex("#8ae977"))
		.build(&mut commands);


	/*
	TODO: make mouse constraint an actual constraint so it applies forces to bodies
		- update error handling so constraints aren't deleted when broken; maybe just don't solve them?
			- throw warnings vs silently fail vs panic
				- unfortunately, can't really let user wrap engine funcs for them to handle errors, so maybe full-on panic and force them to remove constraints?
				- but then user must have reference to constraint when they might remove a body, which isn't always practical or ergonomic or fast
			- also stop render updates, so when a constraint breaks, the renderer keeps constraint on screen but doesn't change position, encouraging proper removal
	TODO: add gear constraint
		v_a = -v_b, where v is the tangent velocity of a point outside the center of the body
		alternatively, calculate max radius r of body and use that as the point
	*/
	
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
		body_a: body_a_id,
		body_a_offset: Vec2::new(-25.0, -25.0),

		body_b: pin_id,

		length: 100.0,

		..Default::default()
	};
	let _fixed_dist = DistanceRenderBuilder::new(fixed_dist)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
}


// Mouse input
#[derive(Resource)]
struct MouseBody(Option<(Entity, Vec2)>);

fn handle_mouse(mouse_buttons: Res<ButtonInput<MouseButton>>, mut commands: Commands, mut mouse_state: ResMut<MouseBody>, camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, window: Single<&Window, With<PrimaryWindow>>, mut bodies: Query<(Entity, &mut Body)>) {
	// Moving main spring constraint by clicking on window
	if window.cursor_position().is_none() { return; } // cursor not in window or not clicking
	let position = window.cursor_position().unwrap(); // guaranteed successful unwrap
	let (camera, camera_transform) = camera.single().expect("camera should be in world");
	let mouse_world_pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();

	if mouse_buttons.just_released(MouseButton::Left) {
		mouse_state.0 = None;
	}

	
	if mouse_state.0.is_none() {
		for (entity, body) in bodies.iter() {
			if body.contains_point(mouse_world_pos) {
				if mouse_buttons.pressed(MouseButton::Left) { // Drag left clicked body
					let offset = body.position - mouse_world_pos;
					mouse_state.0 = Some((entity, offset));
					break;
				}
				if mouse_buttons.pressed(MouseButton::Right) { // Despawn right clicked body
					commands.entity(entity).try_despawn();
				}
			}
		}
	}

	if mouse_buttons.pressed(MouseButton::Left) && let Some(mouse_state) = mouse_state.0 {
		let (_, mut body) = bodies.get_mut(mouse_state.0).unwrap();
		body.set_position(mouse_world_pos + mouse_state.1);
		body.velocity = Vec2::ZERO;
	}
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