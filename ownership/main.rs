fn main() {
    ownership();
}

fn ownership() {
    let my_string = String::from("My name");
    
    // Ownership
    print_string(my_string);

    // It'll fail
    // println!("{}", my_string);
}

fn print_string(string:String) {
    println!("{}", string);
}