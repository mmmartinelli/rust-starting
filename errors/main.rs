fn main() {
    errors();
    // intentional_error();
}

fn errors() {
    match result_success() {
        Ok(s) => println!("The result is: {}", s),
        Err(n) => println!("Error code: {}", n)
    };

    match result_error() {
        Ok(s) => println!("The result is: {}", s),
        Err(n) => println!("Error code: {}", n)
    };
}

fn intentional_error() {
    panic!("This is an intentional error!");
}

fn result_success() -> Result<String, u8> {
    Ok(String::from("Success!"))
}

fn result_error() -> Result<String, u8> {
    Err(99)
}