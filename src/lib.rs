use std::io;

pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0 )
}

 fn main() {
    println!("Enter a year to check if it  is a leap");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let year: u64 = input.trim().parse().expect("Please enter a valid year");

    if is_leap_year(year) {
        println!("{} is a leap year", year)
    } else{
        println!("{} is not a leap", year)
    }
}
