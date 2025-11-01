use std::io;
use rand::Rng;


fn main()
{
    loop
    {
    println!("Guess the numbers");
    println!("Please input your number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new(); // mutable

    io::stdin().read_line(&mut guess).expect("fail to read!");

    println!("You guess : {guess}")
    }

}
