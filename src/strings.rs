// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // str
    let hello = "Hello, the";

    // String
    let mut world = String::from("World ");
    // Push Char to String (not str!)
    world.push('i');
    // Push a string to...a String (again it's not str)
    world.push_str("s Big");

    println!("{} {}", hello, world);

    // Get length
    println!("Length of Hello: {}", hello.len());

    // Get capacity in bytes
    println!("Capacity: {}", world.capacity());

    // Contains
    println!("Contains 'Big'? {}", world.contains("Big"));

    // Replace
    println!("Replace: {}", world.replace("Big", "Small"));

    // Is empty?
    println!("Is empty? {}", world.is_empty());
}
