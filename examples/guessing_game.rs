/// Generates a random integer between 0 and 255.
///
/// Not a relevant part of this example use the `rand` crate for better random
/// numbers.
fn random() -> u32 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos >> 4) & 0xff
}

fn main() {
    let mut num = random();
    println!("Try guess the number I am thinking of 😃 ...");
    println!("  (hint: it's between 0 and 255)\n");

    loop {
        let guess: u32 = casual::prompt("Enter your guess: ").get();

        if guess < num {
            println!("Too low!");
        } else if guess > num {
            println!("Too high!");
        } else {
            println!("You got it!");
            println!("The number was: {}\n", num);

            if casual::confirm("Do you want to play again?") {
                num = random();
            } else {
                break;
            }
        }
    }
}
