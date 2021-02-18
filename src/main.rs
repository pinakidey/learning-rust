#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(unreachable_patterns)]
#![allow(ellipsis_inclusive_range_patterns)]

extern crate rand;
extern crate payload;

mod basics;
mod formatting;
mod combination_lock;
mod data_structures;
mod collections;
mod string_char;
mod functions;
mod traits;
mod lifetime_borrowing;
mod circular_reference;
mod circular_reference_resolution;
mod concurrency;


fn main() {
    /* Toggle comment to see output */
    //formatting::run();
    //basics::run();
    //combination_lock::run();
    //data_structures::run();
    //collections::run();
    //string_char::run();
    //functions::run();
    //traits::run();
    //lifetime_borrowing::run();
    //circular_reference::run();
    //circular_reference_resolution::run();
    //concurrency::run();

    println!("This is a message from the payload crate");
    println!("{}", payload::greetings::english::hello());
}
