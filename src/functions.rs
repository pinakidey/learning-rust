pub fn run() {
    // Square and mutate
    fn square (x: &mut i32) -> i32 {
        *x = *x * *x;
        return *x;
    }

    let mut x = 2;
    println!("{}^2={}", x.to_owned(), square(&mut x));
    println!("{}^2={}", x.to_owned(), square(&mut x));
    println!("{}^2={}", x.to_owned(), square(&mut x));
    println!("{}^2={}", x.to_owned(), square(&mut x));
    // println!("{}^2={}", x.to_owned(), square(&mut x)); //PANICS

    for i in 0..=10 {
        println!("2^{}={}", i, 2i32.pow(i))
    }

    // Add a .len() behavior to struct Line
    struct Line {
        start: (f64, f64),
        end: (f64, f64)
    }

    impl Line {
        fn len(&self) -> f64 {
            let dy = self.end.1 - self.start.1;
            let dx = self.end.0 - self.start.0;
            let length = (dx*dx + dy*dy).sqrt();
            return length;
        }
    }

    let l = Line {start:(0.0,1.0), end:(2.0,3.0)};
    println!("Length {:.2}", l.len());


    // Closure: a function defined in-line
    let _plus_one = |x:i32| -> i32 { x + 1};
    // Return ownership
    let _print = |x:i32| -> i32 {
        println!("{}", x);
        x
    };

    // Higher-order functions (Generators) [a function that takes a function and returns another function]

    // Function returning function
    fn greater_than(limit: i32) -> impl Fn(i32) -> bool {
        move |x| x > limit
    }
    let above_limit = greater_than(500);
    println!("Is 600 greater than 500? {}", above_limit(600));

    // Function taking a function
    let is_even = |x:i32| -> bool {x%2 == 0};
    fn check(func: fn(i32) -> bool, val: i32) -> bool {
        return func(val);
    }
    println!("is_even({}) = {}", 13, check(is_even, 13));

    // Sum of even-squares of numbers up to 500
    let sum:i32 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < 500)
        .filter(|x| is_even(*x))
        //.fold(0, |sum, x| sum + x); /*fold() is similar to reduce is JS*/
        .sum();

    println!("{}", sum);
}