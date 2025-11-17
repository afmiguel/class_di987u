use std::any::Any;

trait Animal{
    fn make_noise(&self);
    fn get_name(&self) -> String;
    fn get_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog{
    fn make_noise(&self){
        println!("Wolf!");
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }

    fn get_any(&self) -> &dyn Any{
        self
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
    is_sleeping: bool,
}

impl Animal for Cat{
    fn make_noise(&self){
        println!("Meow!");
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }

    fn get_any(&self) -> &dyn Any{
        self
    }
}

#[derive(Debug)]
struct Duck {
    name: String,
    can_fly: bool,
}

impl Animal for Duck{
    fn make_noise(&self){
        println!("Quack!");
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }

    fn get_any(&self) -> &dyn Any{
        self
    }
}

fn main() {
    let farm: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog{
            name: "Totó".to_string(),
            age: 5,
        }),
        Box::new(Cat{
            name: "Fifi".to_string(),
            is_sleeping: true,
        }),
        Box::new(Duck{
            name: "Donald".to_string(),
            can_fly: false,
        }),

    ];

    println!("--- Welcome to the Farm with Enums ---");
    for animal in &farm {
        print!("{} make:", animal.get_name());
        animal.make_noise();

        println!("É dog? -> {:?}", animal.get_any().downcast_ref::<Dog>());
        println!("É cat? -> {:?}", animal.get_any().downcast_ref::<Cat>());
        println!("É duck? -> {:?}", animal.get_any().downcast_ref::<Duck>());

        if let Some(cachorro) = animal.get_any().downcast_ref::<Dog>(){
            println!("{} é um cachorro e a idade é {}", animal.get_name(), cachorro.age);
        }

        println!("--------------------");
    }
}

