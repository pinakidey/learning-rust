use std::fmt::{Debug, Formatter, Result, Pointer};
use std::ops::{Add, Deref};

pub fn run() {
    // trait is similar to interface
    trait Animal {
        // <S: Into<String>>(name: S)
        fn create(name: &'static str) -> Self where Self: Sized;
        fn name(&self) -> &'static str;
        fn talk(&self) -> String {
            format!("{} cannot talk.", self.name())
        }
    }
    #[derive(Debug)]
    struct Human {
        name: &'static str
    }
    #[derive(Debug)]
    struct Cat {
        name: &'static str
    }

    impl Animal for Human {
        fn create(name: &'static str) -> Human {
            Human{name}
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) -> String {
            return format!("Hi, my name is {}", self.name)
        }
    }

    impl Drop for Human {
        fn drop(&mut self) {
            println!("{} is now dead.", self.name);
        }
    }

    impl Animal for Cat {
        fn create(name: &'static str) -> Cat {
            Cat{name}
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) -> String {
            return format!("Meow")
        }
    }

    let h = Human::create("Pinaki");
    println!("{} says {}", h.name, h.talk());

    let p = Cat::create("Pussy");
    println!("{} says {}", p.name, p.talk());

    // Adding trait to existing data structures
    trait Summable<T> {
        fn sum(&self) -> T;
    }

    impl Summable<i32> for Vec<i32> {
        fn sum(&self) -> i32 {
            self.iter().sum()
        }
     }

    let a = vec![1,2,3];
    println!("Sum {}", a.sum());
    println!("Sum {}", a.iter().sum::<i32>());

    #[derive(Debug)]
    struct Circle {
        radius: f64
    }
    #[derive(Debug)]
    struct Square {
        side: f64
    }
    trait Shape {
        fn area(&self) -> f64;
    }
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    /*impl Debug for Circle {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Circle (radius: {})", self.radius)
        }
    }
    impl Debug for Square {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Square (side: +{})", self.side)
        }
    }*/

    // Different way to declare function that take trait parameters
    fn print_info1(shape: impl Shape + Debug){
        println!("Area of {:?} is {:.2}", shape, shape.area());
    }

    // trait-bound syntax, concise for multiple arguments
    fn print_info2<T: Shape + Debug>(shape1: T, shape2: T) {
        println!("Area of {:?} is {:.2}", shape1, shape1.area());
        println!("Area of {:?} is {:.2}", shape2, shape2.area());
    }

    fn print_info3<T>(shape: T) where T: Shape + Debug
    {
        println!("Area of {:?} is {:.2}", shape, shape.area());
    }

    let circle1 = Circle{radius: 4.0};
    let circle2 = Circle{radius: 8.0};
    let square1 = Square{side: 4.0};

    print_info1(square1);
    print_info2(circle1, circle2);



    // Operator overloading
    #[derive(Debug, PartialEq, Eq)]
    struct Complex<T> {
        re: T,
        im: T
    }
    impl<T> Complex<T> {
        fn new(re: T, im: T) -> Complex<T> {
            Complex::<T> {re, im}
        }
    }

    impl<T: Add<Output=T>> Add for Complex<T> {
        type Output = Complex<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Complex{re: self.re+rhs.re, im: self.im+rhs.im}
        }
    }

    /*impl<T: PartialEq> PartialEq for Complex<T> {
        fn eq(&self, rhs: &Self) -> bool {
            (self.re == rhs.re) && (self.im == rhs.im)
        }
    }*/

    // Similarly AddAssign, Neg can also be overloaded
    // Full equality cannot always be supported for float, e.g. NAN != NAN.
    // So Partial equality (std::cmp::PartialEq) needs to be supported.

    let c1 = Complex::new(1.0,2.0);
    let c2 = Complex::new(3.0,4.0);

    println!("{:?} and {:?} are equal? {}", c1, c2, c1==c2);
    println!("{:?}", c1+c2);


    // Static Dispatch

    trait Printable {
        fn format(&self) -> String;
    }

    impl Printable for i32 {
        fn format(&self) -> String {
            format!("i32: {}", *self)
        }
    }

    impl Printable for String {
        fn format(&self) -> String {
            format!("String: {}", *self)
        }
    }

    println!("{} {}", 32.format(), "Pinaki".to_string().format());

    // Monomorphization technique
    // Monomorphization is the process of turning generic code into specific code
    // by filling in the concrete types that are used when compiled.
    fn print_static<T: Printable>(z: T) {
        println!("{}", z.format()) /* Concrete types are filled in at compile time */
    }
    print_static("Pinaki".to_string());

    // Dynamic Dispatch
    fn print(z: &dyn Printable) {
        println!("{}", z.format()) /* Which format() is called is determined at runtime, hence more expensive*/
    }
    print(&"Pinaki".to_string());

    // Sometime, Dynamic Dispatch is the only way
    let shapes: [&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 1.0},
        &Circle{radius: 2.0},
        &Square{side: 2.0},
    ];

    for (i,shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {:.2}", i, shape.area()); /*Dynamic dispatching takes place*/
    }

    // Vector of Different structs
    enum Creature {
        Human(Human),
        Cat(Cat)
    }
    let mut creatures = Vec::new();
    creatures.push(Creature::Human(Human::create("John")));
    creatures.push(Creature::Cat(Cat::create("Pussy")));

    for creature in creatures {
        match creature {
            Creature::Human(h) => println!("{}", h.talk()),
            Creature::Cat(c) => println!("{}", c.talk())
        };
    }
    // Another way
    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human{name: "Jack"}));
    animals.push(Box::new(Cat {name: "Pussy"}));

    for animal in animals.iter() {
        println!("{}", animal.talk());
    }
}