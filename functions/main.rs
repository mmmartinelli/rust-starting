fn main()
{
    println!("Result: {}", sum(8, 2));
}

fn sum(a:i32, b:i32) -> i32
{
    println!("Calculating: {} + {}", a, b);
    a + b
}