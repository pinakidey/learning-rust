#![allow(dead_code)]

use std::mem;

pub fn run() {
    /*Core Data Types*/
    let a: u8 = 123;    // u = unsigned, 8 bits, 0 ~ (2^n - 1)
    println!("{}", a);  // immutable

    let mut b: i8 = -123; // i = signed, 8 bits, -2^(n-1) ~ (2^(n-1) - 1)
    println!("{}", b);

    let c = 123456789; // i32
    println!("c = {} takes up {} bytes", c, mem::size_of_val(&c));

    // usize isize
    let d: isize = 123;
    println!("d:isize = {} takes up {} bytes", d, mem::size_of_val(&d));

    // char
    let e: char = 'x'; //immutable, 32-bit, unicode
    println!("e = {} takes up {} bytes", e, mem::size_of_val(&e));

    // Float f32, f64 (IEEE-754) all-signed
    let f: f32 = 1.23;
    let g = 1.23; // inferred type is f64 by default
    println!("f = {} takes up {} bytes", f, mem::size_of_val(&f));
    println!("g = {} takes up {} bytes", g, mem::size_of_val(&g));

    // bool
    let h: bool = false; // takes 8-bits (waste of 7 bits)
    println!("h = {} takes up {} bytes", h, mem::size_of_val(&h));

    /*Operators*/
    let mut i = 2+3*4; // supports precedence
    //i++; Rust doesn't support ++ or --
    i += 1; // Can't mutate unless i is declared as `mut`
    println!("{}", i);
    println!("{} / {} = {}", i, 3, (i/3));
    println!("{} % {} = {}", i, 3, (i%3));

    // Power
    println!("{} ^ {} = {}", 2, 3, (i32::pow(2, 3)));
    println!("{} ^ {} = {}", 2.5, 3, (f64::powi(2.5, 3))); // integer power
    println!("{} ^ {} = {:.5}", 2.5, "PI", (f64::powf(2.5, std::f64::consts::PI))); // floating point power

    // bitwise operator & | ! ^
    let j = 1 | 2; //01 | 10
    println!("{:b}", j);

    // shift operator << >>
    // to calculate 2^10, take 1 and shift left << 10
    println!("2 ^ 10 is {}", (1<<10));

    // logical operator < > <= >= == !=
    println!("Is PI < 4 ? {}", (std::f64::consts::PI < 4.0));

    /*Scope*/
    // Variables are function scoped or block-scoped

    let a = 5; // redeclaration doesn't throw error
    {
        let a = 4;
        println!("{}", a); //4
    }
    println!("{}", a); //5

    /*Constants*/
    const L:i32 = 2; // type inference doesn't work for const/static, a type must be provided
    // const has no fixed address
    println!("{}", L);

    static mut Z:i32 = 123; // valid for entire duration, represents a location in memory
    // a static variable cannot be mutated even if declared as `mut`
    unsafe
        { // but can be mutated inside an unsafe block
            println!("{}", Z);
            Z = 345;
            println!("{}", Z);
        }

    /*Stack & Heap*/
    // Stack: fast but limited in size, used for short term storage
    // Heap: used for long term storage, have more space
    // To add variable in heap, use Box type
    let x = Box::new(5); // returns a pointer
    println!("{}", *x); // Use a de-referencing pointer when accessing
    println!("x = {} takes up {} bytes", x, mem::size_of_val(&*x));

    // struct
    struct Point {
        x: f64,
        y: f64
    }
    fn origin() -> Point {
        Point{x: 1.0, y: 0.0}
    }
    pub fn stack_and_heap() {
        let p1 = origin();
        let p2 = Box::new(origin()); //Boxing, allocation takes place in the Heap
        println!("p1 takes up {} bytes", mem::size_of_val(&p1));
        println!("p2 takes up {} bytes", mem::size_of_val(&p2)); //64-bit on a 64-bit system
        println!("{}", &*p2.x.to_string());
    }

    stack_and_heap();

    /*Conditions*/
    println!("a is {}", if a%2==0 {"even"} else {"odd"});
    let mut k=1;
    loop { // same as while true
        k*=2;
        println!("{}", k);
        if k>(1<<10) {break};
    }
    for x in 1..10 { // 10 not inclusive
        println!("{}", x);
    }

    for (pos, y) in (30..=40).enumerate() {
        println!("{} {}", pos, y);
    }

    /*Match statement*/
    let country_code = 81;
    // match is like switch in other languages, however, exhaustive
    let country = match country_code {
        81 => "JP",
        91 => "IN",
        1  => "US",
        1..=100 => "unknown",
        _ => "invalid"
    }; /*..= inclusive range*/

    println!("Country: {}, Code: {}", country, country_code);
}