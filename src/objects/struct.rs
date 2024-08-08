#[derive(Debug)]
struct Person{
    name: String,
    job:String,
    age: u8,
}

impl Person{
    //Methods associated to my struct
    fn new(n:String, j:String, a:u8) -> Person {
        Person {name:n, job:j, age:a}
    }

    //Instance
    fn greeting(&self){
        println!("Hi {} !", self.name)
    }
}
pub fn person_attribut(){
    //constructor
    let personne = Person::new("ZxFae".to_owned(), "Developper".to_owned(), 99);
    /*let personne = Person{
        name: String::from("ZxFae"),
        job: String::from("Developper"),
        age: 99,
    };*/
    //println!("{:?}", personne);
    //or
    println!("\n Object : {:#?}", personne);
    personne.greeting();
}