use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bau::{ Body, FixedDistance, Constraint };

#[derive(Component)]
pub struct DistanceRenderPin(Entity);

pub struct DistanceRenderBuilder {
	constraint: FixedDistance,
	stroke: Option<(Color, f32)>,

	height: f32,
	margin: f32,
}
impl DistanceRenderBuilder {
	pub fn new(constraint: FixedDistance) -> Self {
		Self {
			constraint,
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
				center: Vec2::ZERO,
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

		let pin_id = commands.spawn((
			pin,
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id();
		commands.spawn((
			Constraint::FixedDistance(self.constraint),
			shape,
			DistanceRenderPin(pin_id),
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id()
	}
}


pub fn update(query: Query<(&mut Shape, &Constraint, &DistanceRenderPin)>, bodies: Query<&Body>, mut shapes: Query<&mut Transform, With<Shape>>) {
	for (mut shape, constraint, pin_id) in query {
		// Verify it is the correct constraint type & unwrap
		let constraint = match constraint {
			Constraint::FixedDistance(constraint) => constraint,
			_ => panic!("spring constraint render should contain a FixedDistance constraint")
		};
		
		// Update spring path
		let body = bodies.get(constraint.body).expect("body in constraint should be in world");
		
		let new_shape = ShapeBuilder::with(
			&shapes::Polygon {
				closed: false,
				points: vec![ body.position.clone(), constraint.position.clone() ],
			})
			.stroke(shape.stroke.expect("constraint render should have a stroke"))
			.build();

		*shape = new_shape;

		// Update pin location (if the spring ever moves)
		let mut pin = shapes.get_mut(pin_id.0).expect("Constraint pin should be in the world");
		pin.translation.x = constraint.position.x;
		pin.translation.y = constraint.position.y;
	}
}