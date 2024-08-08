mod boucles;
mod objects;
use rustcourses::objects::r#struct::person_attribut;
use crate::boucles::r#loop::X;

//Display, not necessary
fn main() {
	let mut x = X::new(0);
	let mut y = X::new(0);
	x.while_a();
	y.while_modulo();

	person_attribut();
}