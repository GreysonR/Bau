use crate::Id;

pub struct World {
	pub bodies: Vec<Id>,
}

impl World {
	pub fn new() -> Self {
		Self {
			bodies: Vec::new(),
		}
	}
	pub fn add_body(&mut self, body: Id) {
		self.bodies.push(body);
	}
	pub fn remove_body(&mut self, id: Id) {
		self.bodies.retain(|body| *body != id);
	}
}