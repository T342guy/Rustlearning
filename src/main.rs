mod Cat;

use rand::RngExt;
use std::cmp::Ordering;
use std::io;
/// this shit down here
// you dont say?
// quite literally a toilet bowl
/*
fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The number is {}", secret_number);

    println!("Put in the guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    this_is_a_value(); //ass
    referr();
    Cat::what(); //running external file
}
fn this_is_a_value() {
    let cheese= 15;
    println!("The value of your cheese is {}", cheese);
    anothervalue(cheese) // idk i just like cheese okay?
}

fn anothervalue(a: i32) {
    let a = a - 2;
    println!("The value of your apple is {}", a); // ?!?!?!?!
}
// i have no idea what im doing but this is awesome
fn referr() {
    let x = 1;
    let _ilend = iget(&x); // The borrow is passed and ran first <--
    println!("look X marks the {}", x) // this is ran last after the borrow
}
fn iget(deez: &i32) {
    println!("Deez is {}", deez);
}

fn nest() {
    pub fn birb() {
        println!("Birb");
    }
} */

use clap::Parser;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}