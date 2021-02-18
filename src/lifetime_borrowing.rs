use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
pub fn run() {
    let v = vec![1, 2, 3];
    // v owns the vec
    // v is in stack, data in the vec is in heap

    //let v2 = v;
    // Since, in Rust, by design, only one variable binds to a resource,
    // v2 does NOT points to the same vec.
    // Since, `Vec<i32>`, does not implement the `Copy` trait, following line won't be compiled.
    //println!("{:?}", v); // v gets moved or invalidated

    // However, similar operation would work for primitive types,
    // because primitives, being not pointing to anything in the heap, implement full Copy, as it is cheap to do so.

    // Again, Box -ing the primitives would put them in the Heap and such operation would not be permitted


    // Since, taking ownership and then returning the same, is cumbersome, we use "borrowing"

    let print = |x: &Vec<i32>| {
        println!("{:?}", x);
    };

    print(&v); // Borrowing by immutable reference
    println!("{:?}", v);

    let mut a = 40;
    {
        let b = &mut a; // mutable borrowing
        *b = 50; //* dereferences what the reference is pointing to
    }
    println!("{}", a); // won't compile unless closure is used as above, since b borrows a
    //50

    let mut z = vec![1, 2, 3];
    for i in &z {
        println!("{}", i);
    }

    /*Lifetime*/
    struct Person {
        name: Rc<String>
    }
    struct Company<'z> {
        name: String,
        ceo: &'z Person
    }

    impl Person {
        fn new(name: Rc<String>) -> Person {
            Person { name }
        }

        fn greet(&self) {
            println!("Hi, my name is {}", self.name)
        }
    }

    let boss = Person { name: Rc::from("Elon Musk".to_string()) };
    let company = Company { name: "Tesla".to_string(), ceo: &boss };

    println!("{}", company.ceo.name);

    /*Reference-counted variables(Rc), single-threaded reference-counting pointer*/
    let name = Rc::new("Pinaki".to_string()); // move occurs because `name` has type `String`, which does not implement the `Copy` trait
    println!("Name: {}. Strong pointers: {}", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone()); // value moved (changed ownership) here
        println!("Name: {}. Strong pointers: {}", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name: {}", name); //won't compile unless Rc is used
    println!("Name: {}. Strong pointers: {}", name, Rc::strong_count(&name));


    /*Atomic Reference-counted variables(Arc), for multi-thread support*/
    struct Teacher {
        name: Arc<String>,
        state: Arc<Mutex<String>> /*Using Mutex (Mutual exclusion) for Thread-safe mutability*/
    }

    impl Teacher {
        fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Teacher {
            Teacher { name, state }
        }

        fn greet(&self) {
            let mut state = self.state.lock().unwrap();
            state.clear(); //mutating
            state.push_str("Excited");
            println!("Name: {}, State: {}", self.name, state.as_str());

        }
    }
    {
        let name = Arc::new("Dr. Pinaki".to_string());
        let state = Arc::new(Mutex::new("Bored".to_string()));
        let mut teacher = Teacher::new(name.clone(), state.clone());
        let t = thread::spawn(move || {
            teacher.greet();
        });
        println!("Name: {}, State: {}", name, state.lock().unwrap().as_str());
        t.join().unwrap();
    }
}