use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    println!("inpout a number");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("inpout a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to reaf");


        println!("guessed: {guess}");

        let guess: u32 = guess.trim().parse().expect("enter only a number");


        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}


