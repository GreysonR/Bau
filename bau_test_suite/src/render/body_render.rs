use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bau::{ Body };

pub struct BodyRenderBuilder {
	body: Body,
	fill: Option<Color>,
	stroke: Option<(Color, f32)>,
}
impl BodyRenderBuilder {
	pub fn new(body: Body) -> Self {
		Self {
			body,
			fill: None,
			stroke: None,
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
	pub fn build(self, commands: &mut Commands) -> Entity {
		assert!(self.fill.is_some() || self.stroke.is_some(), "Body should have a fill or a stroke before building");

		let polygon = shapes::Polygon {
			closed: true,
			points: self.body.vertices.clone(),
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

pub fn update(query: Query<(&Body, &mut Shape)>) {
	for (body, mut shape) in query {
		let polygon = shapes::Polygon {
			closed: true,
			points: body.vertices.clone(), // yikes
			..Default::default()
		};

		*shape = match (shape.fill, shape.stroke) {
			(Some(fill), Some(stroke)) => ShapeBuilder::with(&polygon).fill(fill).stroke(stroke).build(),
			(Some(fill), None) => ShapeBuilder::with(&polygon).fill(fill).build(),
			(None, Some(stroke)) => ShapeBuilder::with(&polygon).stroke(stroke).build(),
			(None, None) => unreachable!(),
		};
	}
}