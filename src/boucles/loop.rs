
pub struct X {
	pub n: usize,
}

impl X {
	pub fn new(n: usize) -> Self {
		Self { n }
	}
	pub fn while_a(&mut self) {
		while self.n < 10 {
			println!("while a {}", self.n);
			//basic incrementation
			self.n += 1;
		}
	}
	pub fn while_modulo(&mut self){
		while  self.n < 10 {
			self.n += 1;
			//even
			if self.n %2 == 0{
				println!("modulo {}", self.n);
			}
			/* odd
			if self.n %2 == 1{
				println!("modulo {}", self.n);
			}
			*/
		}
	}
}
