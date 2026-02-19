use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bau::{ Body };

pub struct BodyRenderBuilder {
	body: Body,
	fill: Option<Color>,
	stroke: Option<(Color, f32)>,
	radius: f32,
}
impl BodyRenderBuilder {
	pub fn new(body: Body) -> Self {
		Self {
			body,
			fill: None,
			stroke: None,
			radius: 5.0,
		}
	}
	#[allow(unused)]
	pub fn fill(mut self, color: Color) -> Self {
		self.fill = Some(color);
		self
	}
	#[allow(unused)]
	pub fn stroke(mut self, stroke: (Color, f32)) -> Self {
		self.stroke = Some(stroke);
		self
	}
	#[allow(unused)]
	pub fn radius(mut self, radius: f32) -> Self {
		self.radius = radius;
		self
	}
	pub fn build(self, commands: &mut Commands) -> Entity {
		assert!(self.fill.is_some() || self.stroke.is_some(), "Body should have a fill or a stroke before building");

		let polygon = shapes::Polygon {
			closed: true,
			points: self.body.vertices.iter().map(|vertex| {
				vertex - self.body.position
			}).collect(), // yikes
			..Default::default()
		};

		let shape = match (self.fill, self.stroke) {
			(Some(fill), Some(stroke)) => ShapeBuilder::with(&polygon).fill(fill).stroke(stroke).build(),
			(Some(fill), None) => ShapeBuilder::with(&polygon).fill(fill).build(),
			(None, Some(stroke)) => ShapeBuilder::with(&polygon).stroke(stroke).build(),
			(None, None) => unreachable!(),
		};

		commands.spawn((
			self.body,
			shape,
			Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
				.with_rotation(Quat::from_rotation_z(0.0)),
		)).id()
	}
}

pub fn update(query: Query<(&Body, &mut Transform)>) {
	for (body, mut transform) in query {
		transform.translation = body.position.extend(transform.translation.z);
		transform.rotation = Quat::from_rotation_z(body.angle);
	}
}