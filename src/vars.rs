// Note: Variables in Rust are immutable by default, so you can't reassign them.

pub fn run() {
    let name = "Sazz";
    // mut: make variables mutable
    let mut age = 14;

    println!("My name is {} and I'm {}", name, age);
    age = 15;
    println!("My name is {} and I'm {}", name, age);

    // Define constant variable
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Multiple variables
    let (my_name, my_age) = ("Sazz", 14);

    println!("{} is {}", my_name, my_age);
}
