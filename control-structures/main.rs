fn main() {
    println!("# Entrance check...");
    is_allowed(15, false);
    is_allowed(15, true);
    is_allowed(31, false);

    println!("# Gender check...");
    check_gender('m');
    check_gender('f');

    println!("# Multiplication table...");
    multiplication_table(5);

    println!("# Language purpose...");
    language_purpose("Kotlin");
}

fn is_allowed(age:u8, member:bool) {
    if age >= 18
    {
        println!("Yeah, you're allowed!");
    }
    else if member
    {
        println!("OK! Show me your ID and you'll be allowed!");
    }
    else
    {
        println!("Hey! Try again in {} years!", 18 - age);
    }
}

fn check_gender(gender:char) {
    let gender_name = if gender == 'm' { "male" } else { "female" }; 
    println!("You are {}!", gender_name);
}

fn multiplication_table(multiplier:u8) {
    for i in 1..=10 {
        println!("{} X {} = {}", multiplier, i, multiplier * i);
    }
    
    // let mut count:u8 = 0;
    // while count < 10 {
    //     count += 1;
    //     println!("{} X {} = {}", multiplier, count, multiplier * count);
    // }

    // count = 0;
    // loop {
    //     count += 1;
    //     println!("{} X {} = {}", multiplier, count, multiplier * count);
        
    //     if count == 10 { break; }
    // }
}

fn language_purpose(language:&str) {
    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unknown"
    };

    println!("The purpose of language {} is {}", language, purpose);
}