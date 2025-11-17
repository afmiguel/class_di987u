// src/main.rs (final version)
struct Dog {
    name: String,
    age: u8,
}

struct Cat {
    name: String,
    is_sleeping: bool,
}

struct Duck {
    name: String,
    can_fly: bool,
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
        Animal::Dog(Dog{
            name: "Totó".to_string(),
            age: 5,
        }),
        Animal::Cat(Cat{
            name: "Fifi".to_string(),
            is_sleeping: true,
        }),
    ];

    println!("--- Welcome to the Farm with Enums ---");
    for animal in &farm {
        animal.make_noise();

        // Using `match` to access specific behavior.
        // The compiler guarantees we cover all Animal types!
//    match animal {
        // Using pattern matching to call specific methods
        // TODO
//    }
        println!("--------------------");
    }
}

