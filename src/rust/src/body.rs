use crate::{Vec2, Geo, Id};

pub struct Body {
	pub id: Id,
	vertices: Vec<Vec2>,
	position: Vec2,
	angle: Geo,
	velocity: Vec2,
	
	pub is_static: bool,
}

impl Body {
	// constructors
	pub fn new(vertices: Vec<Vec2>, position: Vec2, is_static: bool) -> Body {
		assert!(vertices.len() >= 3); // There should be at least 3 vertices for a valid body
		
		let mut body = Body {
			id: rand::random(),
			vertices,
			position: Vec2::new(0.0, 0.0),
			velocity: Vec2::new(0.0, 0.0),
			angle: 0.0,

			is_static,
		};

		body.translate_position(position);

		body
	}
	pub fn rectangle(width: Geo, height: Geo, position: Vec2, is_static: bool) -> Body {
		let half_width = width / 2.0;
		let half_height = height / 2.0;
		let vertices = vec![
			Vec2::new(-half_width, -half_height), // top left
			Vec2::new( half_width, -half_height), // top right
			Vec2::new( half_width,  half_height), // bottom right
			Vec2::new(-half_width,  half_height), // bottom left
		];
		Body::new(vertices, position, is_static)
	}

	// getters
	pub fn get_angle(&self) -> Geo { self.angle }
	pub fn get_position(&self) -> &Vec2 { &self.position }
	pub fn get_vertices(&self) -> &Vec<Vec2> { &self.vertices }
	pub fn get_velocity(&self) -> &Vec2 { &self.velocity }
	
	// setters
	pub fn set_position(&mut self, position: Vec2) {
		self.translate_position(position - self.position);
	}
	pub fn translate_position(&mut self, translation: Vec2) {
		self.position += translation;
		
		for vertex in self.vertices.iter_mut() {
			*vertex += translation;
		}
	}
	pub fn set_velocity(&mut self, velocity: Vec2) {
		self.velocity = velocity;
	}
	pub fn apply_velocity(&mut self, force: &Vec2) {
		self.velocity += force;
	}
}

impl PartialEq for Body {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Body {}
