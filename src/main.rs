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

fn print_people_list(peoplevec: &Vec<Box<dyn PrintToTerm>>) -> i32 {
    println!("Fellow people of the HelloWorld:");

    for person in peoplevec.iter() {
        print!("\t - ");
        person.prn();
    }

    peoplevec.len() as i32
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

    println!("Total people count: {}",
             print_people_list(&people));
}