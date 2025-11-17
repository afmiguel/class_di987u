// src/main.rs (final version)
#[allow(dead_code)]
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn wag_tail(&self) {
        println!("* {} wags its tail happily! *", self.name);
    }
}

struct Cat {
    name: String,
    is_sleeping: bool,
}

impl Cat {
    fn purr(&self) {
        if self.is_sleeping {
            println!("* {} is sleeping and purring... zZz... prrrr... *", self.name);
        } else {
            println!("* {} looks at you and purrs: Prrrr! *", self.name);
        }
    }
}

// Using an enum Animal to represent different types of animals
enum Animal{
    Dog(Dog),
    Cat(Cat),
}

// Implementing methods for the Animal enum
impl Animal{
    fn make_noise(&self){
        match self{
            Animal::Dog(cao) => {
                cao.wag_tail();
            },
            Animal::Cat(gato) => {
                gato.purr();
            }
        }
    }
}

fn main() {
    let farm = vec![
        // TODO: Create instances of Dog and Cat
    ];

    println!("--- Welcome to the Farm with Enums ---");
    for animal in &farm {
        animal.make_noise();

        // Using `match` to access specific behavior.
        // The compiler guarantees we cover all Animal types!
        match animal {
            // Using pattern matching to call specific methods
            // TODO
        }
        println!("--------------------");
    }
}

