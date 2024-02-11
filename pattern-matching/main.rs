fn main() {
    pattern_matching();
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1      => "One!",
            2 | 3  => "Greater than one and less than four!",
            4..=10 => "Greater than three and less than eleven!",
            _ if x % 2 == 0 => "Greater than eleven and even!",
            _      => "Greater than eleven and odd!"
        });
    }
}