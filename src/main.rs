use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // references are immutable by default, here we need a mutable reference,
        // so use `&mut guess` rather than `&guess`
        // read_line also return a result of type io::Result ( There is a generic
        // Result as well as specific versions for submodules as in io::Result )
        // Result types are enumerators ( enums ). The variants are Ok, Err

        //let guess: u32 = guess.trim().parse()
        //    .expect("Enter a number!!");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // Ordering is another enum, like Result
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
            Ordering::Greater => println!("Too big!!"),
        }
    }
}
