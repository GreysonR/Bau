use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bau;

mod render;


fn main() {
	println!("Bau: {}", bau::add(2, 2));
	
	App::new()
		.add_plugins(DefaultPlugins)
		// .add_systems(Update, print_mouse_position)
		.add_plugins(render::Render)
		.run();

}

#[allow(unused)]
fn print_mouse_position(window: Single<&Window, With<PrimaryWindow>>) {
	if let Some(position) = window.cursor_position() {
		println!("mouse is at ({}, {})", position.x, position.y);
	}
	// else not in window
}
