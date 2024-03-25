use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1 ..=100);





    loop {
        println!("Please input your guess.");



        println!("The secret number is: {}", secret_number);






        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("에러");

        println!("You guessed: {}", guess);


        //let guess: u32 = guess.trim().parse().expect("파싱 에러");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
