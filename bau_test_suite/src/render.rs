use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowCloseRequested, };
use bevy_prototype_lyon::prelude::*;

pub struct Render;
impl Plugin for Render {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(ShapePlugin)
			.add_systems(Startup, (setup_render, add_polygon).chain())
			// .add_systems(Update, print_time)
			.add_systems(Update, update_shape)
			.add_systems(Update, handle_quit)
			;
	}
}

#[derive(Component)]
struct EngineShape;


#[derive(Component)]
struct Velocity(Vec2);


#[derive(Component)]
struct Gravity(Vec2);

fn color_hex(hex: &str) -> Color {
	Color::Srgba(Srgba::hex(hex).unwrap())
}

fn setup_render(mut commands: Commands) {
	commands.spawn((Camera2d, Msaa::Sample4));
	commands.insert_resource(ClearColor(color_hex("#0C4440")));
}

fn handle_quit(keys: Res<ButtonInput<KeyCode>>, mut close_events: MessageWriter<WindowCloseRequested>, windows: Query<Entity, With<Window>>) {
	if keys.just_pressed(KeyCode::KeyQ) {
		let window = windows.single();
		if let Err(_) = window { return; }
		let window = window.unwrap();
		close_events.write(WindowCloseRequested { window });
	}
}

fn add_polygon(mut commands: Commands) {
	let points = vec![
		Vec2::new(0.0, 0.0),
		Vec2::new(100.0, 0.0),
		Vec2::new(100.0, 100.0),
		Vec2::new(0.0, 100.0),
	];

	let position = Vec2 { x: -400.0, y: 50.0 };
	let polygon = shapes::RoundedPolygon {
		points,
		radius: 5.0,
		..Default::default()
	};
	let shape = ShapeBuilder::with(&polygon)
		// .fill(color_hex("#F4FDD9"))
		.stroke((color_hex("#F0A152"), 4.0))
		.build();


	commands.spawn((
		shape,
		Transform::from_translation(position.extend(0.0))
			.with_rotation(Quat::from_rotation_z(PI * 0.25)),
		Velocity(Vec2::new(200.0, 500.0)),
		Gravity(Vec2::new(0.0, -600.0)),
		EngineShape
	));
}

fn update_shape(query: Query<(&mut Transform, &mut Velocity, &Gravity), With<EngineShape>>, time: Res<Time>) {
	let delta = time.delta().as_secs_f32();
	for (mut transform, mut velocity, gravity) in query {
		// Apply forces
		let translation = &mut transform.translation;
		velocity.0 += gravity.0 * Vec2::splat(delta);

		// Apply air friction
		let air_friction: f32 = 0.4;
		velocity.0 *= Vec2::splat((1.0 - air_friction).powf(delta));

		if velocity.0.length() < 0.01 {
			velocity.0 = Vec2::splat(0.0);
		}

		// Apply velocity
		*translation += (velocity.0 * Vec2::splat(delta)).extend(0.0);

		// Apply constraints
		let floor = -300.0;
		if translation.y < floor {
			let restitution = 0.5;
			velocity.0.y = (-velocity.0.y * restitution).max(0.0);
			translation.y = translation.y.max(floor);
		}
		// info!("transform: {}, {}", transform.translation.x, transform.translation.y);
	}
}

fn print_time(time: Res<Time>) {
	info!("Current time: {:.2}; fps: {:.2}", time.elapsed_secs(), 1.0 / time.delta_secs());
}