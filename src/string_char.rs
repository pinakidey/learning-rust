use rand::Rng;
use std::io::{stdin, Error};
use std::num::ParseIntError;

pub fn run() {

    // utf-8
    let s: &'static str = "Hello World"; // &str = string slice, 
    // static = directly accessible inside the program
    //s = "abc"; // immutable
    println!("{}", s);
    for c in s.chars().rev() {
        print!("{}",c);
    }
    println!();
    println!("{}", s);

    let chars = s.chars();
    println!("{:?}", chars);

    if let Some(c) = s.chars().nth(0) {
        println!("first letter is {}", c);
    }

    // String (heap allocated)
    let mut letters = String::new();
    let mut l = 'a' as u8;
    while l < ('z' as u8) {
        letters.push(l as char);
        l +=1;
    }
    println!("{}", letters);

    // &str <> String conversion
    let u:&str = &letters; //deref conversion
    println!("{}", u);

    // concat
    let f = u.to_owned() + " ";
    println!("{}", f + &letters);

    let mut v = "hello world".to_string();
    let mut w = String::from("hello world").replace("hello", "goodbye");
    v.push('!');
    println!("{} {}", v, w);

    // format!() macro
    println!("{}", format!("{0}, {1} {0}. Hi Mr. {}", "Bond", "James"));

    // Number guessing game

    let number = rand::thread_rng().gen_range(1..101);
    let mut score = 100;
    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess <1 || guess > 100 {println!("Out of range.")}
                        if guess < number {println!("Your guess is low")}
                        if guess > number {println!("Your guess is high")}
                        score -=1;
                        if guess == number {println!("That's a Match. Your Score {}/100", score); break;}
                        if score == 0 {println!("Game Over."); break;}
                    }
                    Err(e) => {println!("Error: {}", e)}
                }
            }
            Err(_) => {continue}
        }
    }
}