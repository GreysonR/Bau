use bevy::{math::VectorSpace, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bau::{ Body, FixedDistance, Constraint };

#[derive(Component)]
pub struct DistanceRender; // marker component

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

		// Line between bodies
		let polygon = shapes::Polygon {
			closed: false,
			points: vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)],
		};
		let shape = ShapeBuilder::with(&polygon)
			.stroke(stroke)
			.build();

		commands.spawn((
			Constraint::FixedDistance(self.constraint),
			DistanceRender,
			shape,
			Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id()
	}
}


pub fn update(query: Query<(Entity, &mut Shape, &Constraint, &DistanceRender)>, mut commands: Commands, bodies: Query<&Body>) {
	for (entity, mut shape, constraint, _) in query {
		// Verify it is the correct constraint type & unwrap
		let constraint = match constraint {
			Constraint::FixedDistance(constraint) => constraint,
			_ => panic!("distance constraint render should contain a FixedDistance constraint")
		};
		
		// Update spring path
		if constraint.body_a.is_none() || constraint.body_b.is_none() {
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
		let body_query = bodies.get_many([constraint.body_a.unwrap(), constraint.body_b.unwrap()]);
		if body_query.is_err() {
			commands.entity(entity).try_despawn();
			warn!("Removed FixedDistanceRender {entity}: at least one of its bodies wasn't in the world");
			return;
		}
		let [body_a, body_b] = body_query.unwrap();
		
		let new_shape = ShapeBuilder::with(
			&shapes::Polygon {
				closed: false,
				points: vec![
					body_a.position + constraint.body_a_offset.rotate(Vec2::from_angle(body_a.angle)),
					body_b.position + constraint.body_b_offset.rotate(Vec2::from_angle(body_b.angle)),
				],
			})
			.stroke(shape.stroke.expect("constraint render should have a stroke"))
			.build();

		*shape = new_shape;
	}
}