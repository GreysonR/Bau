use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bau::{ Body, Spring, Constraint };

// Spring rendering stuff
#[derive(Component)]
pub struct SpringRender {
	height: f32,
	length: f32, // Spring initial length
	margin: f32,
}
impl SpringRender {
	fn get_points(&self, start: &Vec2, end: &Vec2) -> Vec<Vec2> { // dir is the vector from the start of the spring to the 
		// Basic calculations
		let difference = end - start;
		let dir = difference.normalize();
		let length = difference.length();
		let margin = self.margin.min(length * 0.5 - 0.01);
		let n_pts = ((self.length - 2.0 * margin) / 6.0).floor() as i32;
		
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

		// Translate points to real positions
		points.iter_mut().for_each(|point| {
			*point = point.rotate(dir) + start;
		});

		points
	}
}


#[derive(Component)]
pub struct SpringRenderPin(Entity);

pub struct SpringRenderBuilder {
	spring: Spring,
	stroke: Option<(Color, f32)>,

	height: f32,
	margin: f32,
}
impl SpringRenderBuilder {
	pub fn new(spring: Spring) -> Self {
		Self {
			spring,
			stroke: None,
			height: 3.0,
			margin: 6.0,
		}
	}
	#[allow(unused)]
	pub fn stroke(mut self, stroke: (Color, f32)) -> Self {
		self.stroke = Some(stroke);
		self
	}
	#[allow(unused)]
	pub fn height(mut self, height: f32) -> Self {
		self.height = height;
		self
	}
	#[allow(unused)]
	pub fn margin(mut self, margin: f32) -> Self {
		self.margin = margin;
		self
	}

	pub fn build(self, commands: &mut Commands) -> Entity { // TODO: consider generalizing this, and/or turning this method into one that takes in options & the spring rather than a whole builder
		let stroke = self.stroke.expect("Body should have a stroke before building");

		// Pin at end of spring
		let pin = ShapeBuilder::with(
			&shapes::Circle {
				center: self.spring.position.clone(),
				radius: stroke.1 * 1.2,
				..Default::default()
			})
			.fill(stroke.0.clone())
			.build();


		// Jagged spring line
		let polygon = shapes::Polygon {
			closed: false,
			points: vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)],
		};
		let shape = ShapeBuilder::with(&polygon)
			.stroke(stroke)
			.build();
		let spring_render = SpringRender {
			height: self.height,
			margin: self.margin,
			length: self.spring.length,
		};

		let pin_id = commands.spawn((
			pin,
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id();
		commands.spawn((
			Constraint::Spring(self.spring),
			shape,
			SpringRenderPin(pin_id),
			spring_render,
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id()
	}
}


pub fn update(query: Query<(&SpringRender, &mut Shape, &Constraint, &SpringRenderPin)>, bodies: Query<&Body>, mut shapes: Query<&mut Transform, With<Shape>>) {
	for (spring_render, mut shape, constraint, pin_id) in query {
		// Verify it is a spring & unwrap
		let spring = match constraint {
			Constraint::Spring(spring) => spring,
			_ => panic!("spring constraint render should contain a spring")
		};
		
		// Update spring path
		let body = bodies.get(spring.body).expect("body in spring constraint should be in world");
		let points = spring_render.get_points(&spring.position, &body.position);
		
		let new_shape = ShapeBuilder::with(
			&shapes::Polygon {
				closed: false,
				points,
			})
			.stroke(shape.stroke.expect("spring render should have a stroke"))
			.build();

		*shape = new_shape;

		// Update pin location (if the spring ever moves)
		let mut pin = shapes.get_mut(pin_id.0).expect("Spring pin should be in the world");
		pin.translation.x = spring.position.x;
		pin.translation.y = spring.position.y;
	}
}