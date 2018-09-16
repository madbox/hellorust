// Implementing some kind of polymorphism with Traits

pub trait PrintToTerm {
    fn prn(&self);
}

pub struct Peasant {
    pub name: String
}

impl PrintToTerm for Peasant {
    fn prn(&self) {
        println!("Peasant {}", self.name)
    }
}

pub struct Lord {
    pub name: String,
    pub land: String,
}

impl PrintToTerm for Lord {
    fn prn(&self) {
        println!("Lord {} of the {}", self.name, self.land)
    }
}


fn main() {
    let mut people: Vec<Box<dyn PrintToTerm>> = Vec::new();

    let h = Peasant {
        name: String::from("Tankard")
    };

    let h2 = Lord {
        name: String::from("Theodric"),
        land: String::from("Northendale"),
    };

    people.push(Box::new(h));
    people.push(Box::new(h2));

    println!("Fellow people of the HelloWorld:");

    for person in people.iter() {
        print!("\t - ");
        person.prn();
    }
}