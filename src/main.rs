mod boucles;
mod objects;
use rustcourses::objects::r#struct::person_attribut;
use crate::boucles::r#while::X;
use crate::boucles::r#for::*;

//Display, not necessary
fn main() {
	/*
	let mut x = X::new(0);
	let mut y = X::new(0);
	x.while_a();
	y.while_modulo();
	*/

	let array = Array::new([1,2,3,4,5]);
	//array.iteration();
	array.iteration_others();
	//person_attribut();
}