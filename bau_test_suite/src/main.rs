use std::f32::consts::PI;

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
		.add_systems(Update, (handle_mouse, handle_input))
		.run();

}


fn add_bodies(mut commands: Commands) {
	// Add bodies
	let body_a_id = BodyRenderBuilder::new(
			BodyBuilder::rect(50.0, 50.0)
			.position(Vec2::new(200.0, 0.0))
			// .velocity(Vec2::new(-40.0, 0.0))
			.angle(std::f32::consts::PI * 0.2)
			// .mass(1.0)
			.build()
		)
		.fill(color_hex("#F0A152"))
		.build(&mut commands);
	
	
	let body_b_id = BodyRenderBuilder::new(
			BodyBuilder::circle(10.0)
			.position(Vec2::new(0.0, 0.0))
			.is_static(true)
			.build()
		)
		.fill(color_hex("#E35531"))
		.build(&mut commands);
	
	
	let body_c_id = BodyRenderBuilder::new(
			BodyBuilder::rect(30.0, 30.0)
			.position(Vec2::new(200.0, 0.0))
			.velocity(Vec2::new(-40.0, 0.0))
			// .mass(150.0)
			.build()
		)
		.stroke((color_hex("#F0A152"), 1.0))
		.build(&mut commands);
	

	let pin_id = BodyRenderBuilder::new(
			BodyBuilder::circle(3.0)
			.position(Vec2::new(100.0, 0.0))
			.is_static(true)
			.build()
		)
		.fill(color_hex("#c1c1c168"))
		.build(&mut commands);


	let _body_d_id = BodyRenderBuilder::new(
			BodyBuilder::rect(50.0, 50.0)
			.position(Vec2::new(200.0, 400.0))
			// .velocity(Vec2::new(-40.0, 0.0))
			.angle(std::f32::consts::PI * 0.1)
			.friction_air(0.9)
			// .mass(1.0)
			.build()
		)
		.fill(color_hex("#8ae977"))
		.build(&mut commands);


	/*
	TODO: add gear constraint
		v_a = -v_b, where v is the tangent velocity of a point outside the center of the body
			- maybe calculate max radius r of body and use that as the point
	*/
	
	// Add spring constraints
	let _spring = SpringRenderBuilder::new(
			Spring {
				body_a: Some(body_a_id),
				body_a_offset: Vec2::new(25.0, 25.0),
				body_b: Some(body_b_id),

				unstretched_length: 100.0,
				frequency: 2.0,
				damping: 0.01,

				..Default::default()
			}
		)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
	

	let _spring2 = SpringRenderBuilder::new(
		Spring {
			body_a: Some(body_a_id),
			body_a_offset: Vec2::new(-25.0, 25.0),
			body_b: Some(body_c_id),
			body_b_offset: Vec2::new(0.0, -15.0),

			unstretched_length: 50.0,
			frequency: 1.0,
			damping: 0.01,

			..Default::default()
		}
	)
		.stroke((color_hex("#fdf2d9b2"), 2.0))
		.build(&mut commands);


	// Add fixed distance constraint
	let _fixed_dist = DistanceRenderBuilder::new(
			FixedDistance {
				body_a: Some(body_a_id),
				body_a_offset: Vec2::new(-25.0, -25.0),

				body_b: Some(pin_id),

				length: 100.0,

				..Default::default()
			}
		)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);

	
	// Mouse control
	let mouse_body = BodyRenderBuilder::new(
			BodyBuilder::circle(4.0)
			.position(Vec2::new(0.0, 0.0))
			// .velocity(Vec2::new(-40.0, 0.0))
			// .mass(1.0)
			.is_static(true)
			.build()
		)
		.stroke((color_hex("#f9f9f9a9"), 1.0))
		.build(&mut commands);
	let mouse_constraint = DistanceRenderBuilder::new(
			FixedDistance {
				body_a: Some(mouse_body),
				// body_a_offset: Vec2::new(-25.0, -25.0),
				body_b: None,
				length: 0.0,
				..Default::default()
			}
		)
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		.build(&mut commands);
	commands.insert_resource(Mouse {
		body: mouse_body,
		constraint: mouse_constraint,
		holding: None,
	});
}


// Mouse input
#[derive(Resource)]
struct Mouse {
	body: Entity,
	constraint: Entity,
	holding: Option<Entity>
}

fn handle_mouse(mouse_buttons: Res<ButtonInput<MouseButton>>, mut commands: Commands, mut mouse_state: ResMut<Mouse>, camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, window: Single<&Window, With<PrimaryWindow>>, mut bodies: Query<(Entity, &mut Body)>, mut constraints: Query<&mut Constraint>) {
	let mut mouse_constraint = constraints.get_mut(mouse_state.constraint).expect("Mouse constraint not found");
	let mouse_constraint = match &mut *mouse_constraint {
		Constraint::FixedDistance(s) => s,
		_ => panic!("Expected mouse constraint to be FixedDistance")
	};

	if mouse_buttons.just_released(MouseButton::Left) || window.cursor_position().is_none() { // no longer clicking or off window
		mouse_state.holding = None;
		mouse_constraint.body_b = None;
		return;
	}
	let position = window.cursor_position().unwrap(); // already checked; guaranteed successful unwrap
	let (camera, camera_transform) = camera.single().expect("camera should be in world");
	let mouse_world_pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();


	// Update mouse body position
	let (_, mut mouse_body) = bodies.get_mut(mouse_state.body).expect("Mouse body not found");
	mouse_body.set_position(mouse_world_pos);

	// Move static bodies by directly setting their position
	if mouse_buttons.pressed(MouseButton::Left) && let Some(mouse_holding) = mouse_state.holding {
		let (_, mut body) = bodies.get_mut(mouse_holding).unwrap();
		if body.is_static {
			body.set_position(mouse_world_pos + mouse_constraint.body_b_offset);
		}
	}
	
	
	// Find new bodies to hold or delete
	if mouse_state.holding.is_none() {
		for (entity, body) in bodies.iter() {
			if !body.contains_point(mouse_world_pos) || entity == mouse_state.body { continue; }

			if mouse_buttons.pressed(MouseButton::Left) { // Drag left clicked body
				let offset = body.position - mouse_world_pos;
				
				mouse_state.holding = Some(entity);
				mouse_constraint.body_b = Some(entity);
				mouse_constraint.body_b_offset = offset.rotate(Vec2::from_angle(-body.angle + PI));
				// mouse_constraint.body_b_offset = offset;
				
				break;
			}
			else if mouse_buttons.pressed(MouseButton::Right) { // Despawn right clicked body
				commands.entity(entity).try_despawn();
				break;
			}
		}
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
			let velocity = body.get_velocity();
			let impulse = 100.0 * body.mass;
			body.set_velocity(velocity + impulse * intent);
			break; // only apply to 1st body
		}
	}
}