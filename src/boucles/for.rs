pub struct Array {
    pub elements: [i32; 5],
}

impl Array {
    pub fn new(elements: [i32; 5]) -> Self {
        Self { elements }
    }

    pub fn iteration(&self){
        for (index, element) in self.elements.iter().enumerate(){
            println!("index {}- element {}",index,  element);
        }
    }
    //Example
    pub fn iteration_others(&self){
        for i in 1 ..  11{
            for (j, elem) in self.elements.iter().enumerate(){
                println!("{} - {}-{}", i,j, elem);
            }
        }
    }
}