fn main() {
    ownership();
}

fn ownership() {
    let mut owner_string = String::from("My name");

    // Ownership
    print_string(&mut owner_string); 
    // Pointer &owner_string
    // &mut allows changes

    println!("{}", owner_string);
}

fn print_string(borrowing:&mut String) { // &String is a reference to a String in Heap memory
    // &mut allows changes
    borrowing.push_str("!");
    println!("{}", borrowing);
}