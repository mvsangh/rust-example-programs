fn main() {
    // take age in years as input and print them in days,hours minutes and seconds.

    loop {
        println!("\nPlease enter your age: ");
        let mut age = String::new();
        std::io::stdin().read_line(&mut age).unwrap();

        let age: i32 = age.trim().parse().unwrap();

        if age <= 0 {
            println!("Thanks for Calculating!");
            break;
        }
        let days = age * 365;
        let hours = days * 24;
        let minutes = hours * 60;
        let seconds = minutes * 60;

        // Print the results
        println!("Your age in:");
        println!("Days: {} days.", days);
        println!("Hours: {} hours.", hours);
        println!("Minutes: {} minutes.", minutes);
        println!("Seconds: {} seconds.", seconds);
    }
}
