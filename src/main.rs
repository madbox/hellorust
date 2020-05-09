extern crate rand;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;


// Implementing some kind of polymorphism with Traits

pub trait PrintToTerm {
    fn prn(&self) {
        println!("{}", self.get_str())
    }
    fn get_str(&self) -> String;
}

pub struct Peasant {
    pub name: String
}

impl PrintToTerm for Peasant {
    fn get_str(&self) -> String {
        format!("Peasant {}", self.name)
    }
}

pub struct Lord {
    pub name: String,
    pub land: String,
}

impl PrintToTerm for Lord {
    fn get_str(&self) -> String {
        format!("Lord {} of the {}", self.name, self.land)
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

const NAMES: &'static [&'static str] = &["Arne", "Bjørn", "Eirik",  "Geir",  "Gisle", "Gunnar", "Harald",  "Håkon", "Inge", "Ivar", "Knut", "Leif", "Magnus", "Olav", "Rolf", "Sigurd", "Snorre", "Steinar", "Torstein", "Trygve", "Ulf", "Valdemar", "Vidar", "Yngve", "Tankard", "Northendale", "Theodric"];

fn main() {
    let mut people: Vec<Box<dyn PrintToTerm>> = Vec::new();
    let mut rng = thread_rng();


    for _ in 0..rng.gen_range(2,5) {
        let h = Peasant {
            name: String::from(*NAMES.choose(&mut rng).unwrap())
        };
        people.push(Box::new(h));
    };

    let h2 = Lord {
        name: String::from(*NAMES.choose(&mut rng).unwrap()),
        land: String::from("Northendale"),
    };

    people.push(Box::new(h2));

    println!("Total people count: {}",
             print_people_list(&people));

    let myprint = |s: &String| -> String {
	    return format!("This is a function + {}", s);
    };

    let mut somestring = String::from("initstr!");
	
    println!("Fisrt attempt: {}", myprint(&somestring));

    somestring.push_str("+++");

    println!("Second attempt: {}", myprint(&somestring));
}
