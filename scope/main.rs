fn main()
{
    println!("This is MAIN scope!");
    
    first_scope();
    second_scope();
}

fn first_scope()
{
    println!("This is FIRST scope!");
}

fn second_scope()
{
    println!("This is SECOND scope!");

    {
        println!("This is THIRD scope!");
    }
}