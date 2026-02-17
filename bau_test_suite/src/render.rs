use bevy::{ prelude::*, window::WindowCloseRequested };
use bevy_prototype_lyon::prelude::*;

use bau::{ Body, Constraint, Spring };

pub struct Render;
impl Plugin for Render {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(ShapePlugin)
			.add_systems(Startup, (setup_render, init_engine))
			// .add_systems(Update, print_time)
			.add_systems(Update, (render_bodies, render_springs))
			.add_systems(Update, handle_quit)
			;
	}
}

fn handle_quit(keys: Res<ButtonInput<KeyCode>>, mut close_events: MessageWriter<WindowCloseRequested>, windows: Query<Entity, With<Window>>, mut bodies: Query<&mut Body>) {
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

		let mut body = bodies.single_mut().unwrap();
		let impulse = 1000.0 * body.mass;
		body.velocity += impulse * intent;
	}
}

fn color_hex(hex: &str) -> Color {
	Color::Srgba(Srgba::hex(hex).unwrap())
}

fn setup_render(mut commands: Commands) {
	commands.spawn((Camera2d, Msaa::Sample4));
	commands.insert_resource(ClearColor(color_hex("#0C4440")));
}



fn init_engine(mut commands: Commands) {
	let (body_id, spring_id) = bau::add_constraint(&mut commands);
	add_body_polygon(body_id, &mut commands);
	add_spring_polygon(spring_id, &mut commands);
}

fn add_body_polygon(body_id: Entity, commands: &mut Commands) {
	let polygon = shapes::Circle {
		center: Vec2::ZERO,
		radius: 5.0,
		..Default::default()
	};
	let shape = ShapeBuilder::with(&polygon)
		.fill(color_hex("#F0A152"))
		// .stroke((color_hex("#F0A152"), 4.0))
		.build();


	commands.entity(body_id).insert((
		shape,
		Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
			.with_rotation(Quat::from_rotation_z(0.0)),
	));
}

#[derive(Component)]
struct SpringRender {
	height: f32,
	length: f32, // Spring initial length
	margin: f32,
	constraint: Entity,
}
impl SpringRender {
	fn get_points(&self, start: &Vec2, end: &Vec2) -> Vec<Vec2> { // dir is the vector from the start of the spring to the 
		// Basic calculations
		let difference = end - start;
		let dir = difference.normalize();
		let length = difference.length();
		let margin = self.margin.min(length * 0.5 - 0.01);
		let n_pts = ((self.length - 2.0 * margin) / 10.0).floor() as i32;
		
		// Build initial points
		let mut points = Vec::new();
		points.push(Vec2::new(0.0, 0.0));
		points.push(Vec2::new(margin, 0.0));
		for i in 0..n_pts {
			let x = (i as f32 + 0.5) / (n_pts as f32) * (length - 2.0 * margin) + margin;
			let y = ((i % 2) as f32 - 0.5).signum() * self.height;
			points.push(Vec2::new(x, y));
		}
		points.push(Vec2::new(length - margin, 0.0));
		points.push(Vec2::new(length, 0.0));

		// Translate points to match
		points.iter_mut().for_each(|point| {
			*point = point.rotate(dir) + start;
		});

		points
	}
}
fn add_spring_polygon(spring_id: Entity, commands: &mut Commands) {
	let spring_polygon = &shapes::Polygon {
		closed: false,
		points: vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)],
	};
	// let pin = &shapes::Circle {
	// 	center: Vec2::ZERO,
	// 	radius: 2.0,
	// 	..Default::default()
	// };

	let shape = ShapeBuilder::with(spring_polygon)
		// .fill(color_hex("#F4FDD9"))
		.stroke((color_hex("#f4fdd9b2"), 2.0))
		// .add(pin)
		.build();

	let spring_render = SpringRender {
		height: 3.0,
		margin: 6.0,
		length: 100.0, // TODO: get this correctly
		constraint: spring_id,
	};

	commands.entity(spring_id).insert((
		spring_render,
		shape,
	));
}

fn render_bodies(query: Query<(&Body, &mut Transform)>) {
	for (body, mut transform) in query {
		transform.translation = body.position.extend(1.0);
	}
}
fn render_springs(query: Query<(&SpringRender, &mut Shape)>, springs: Query<&Constraint>, bodies: Query<&Body>) {
	for (spring_render, mut shape) in query {
		let constraint = springs.get(spring_render.constraint).expect("spring should be in world"); // TODO: handle unwraps
		let spring = match constraint { Constraint::Spring(spring) => spring, _ => panic!("constraint should be a spring") };
		let body = bodies.get(spring.body).expect("body should be in world");
		let points = spring_render.get_points(&spring.position, &body.position);
		
		let new_shape = ShapeBuilder::with(
			&shapes::Polygon {
				closed: false,
				points,
			})
			.stroke(shape.stroke.expect("spring render should have a stroke"))
			.build();

		*shape = new_shape;
		// shape.path
		// spring_render.update_points(&spring.position, &body.position);
	}
}