use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bau::{ Body, Spring, Constraint };

// Spring rendering stuff
#[derive(Component)]
pub struct SpringRender {
	height: f32,
	unstretched_length: f32, // Spring initial length
	margin: f32,
}
impl SpringRender {
	fn get_points(&self, start: &Vec2, end: &Vec2) -> Vec<Vec2> {
		// Basic calculations
		let difference = end - start;
		let dir = difference.normalize();
		let length = difference.length();
		let margin = self.margin.min(length * 0.5 - 0.01);
		let n_pts = ((self.unstretched_length - 2.0 * margin) / 6.0).floor() as i32;
		
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
			unstretched_length: self.spring.unstretched_length,
		};
		
		commands.spawn((
			Constraint::Spring(self.spring),
			shape,
			spring_render,
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id()
	}
}


pub fn update(query: Query<(&SpringRender, &mut Shape, &mut Constraint)>, bodies: Query<&Body>) {
	for (spring_render, mut shape, mut constraint) in query {
		// Verify it is a spring & unwrap
		let spring = match &mut *constraint {
			Constraint::Spring(spring) => spring,
			_ => panic!("spring constraint render should contain a spring")
		};
		
		// Update spring path
		if spring.body_a.is_none() || spring.body_b.is_none() {
			// Make invisible & return if either body is None
			let new_shape = ShapeBuilder::with(
				&shapes::Polygon {
					closed: false,
					points: vec![Vec2::ZERO, Vec2::ONE], // todo: hide the constraint properly
				})
				.stroke(shape.stroke.expect("constraint render should have a stroke"))
				.build();

			*shape = new_shape;

			return;
		}

		let result = bodies.get_many([spring.body_a.unwrap(), spring.body_b.unwrap()]);
		if result.is_err() { // at least one of the bodies is not in the world anymore, so set that body to None in the constraint
			if bodies.get(spring.body_a.unwrap()).is_err() {
				spring.body_a = None;
			}
			return;
		}
		let [body_a, body_b] = result.unwrap();
		let points = spring_render.get_points(
			&(body_a.position + spring.body_a_offset.rotate(Vec2::from_angle(body_a.angle))),
			&(body_b.position + spring.body_b_offset.rotate(Vec2::from_angle(body_b.angle)))
		);
		
		let new_shape = ShapeBuilder::with(
			&shapes::Polygon {
				closed: false,
				points,
			})
			.stroke(shape.stroke.expect("spring render should have a stroke"))
			.build();

		*shape = new_shape;
	}
}