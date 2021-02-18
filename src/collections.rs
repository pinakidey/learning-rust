use std::collections::{HashMap, HashSet};

pub fn run() {

    /*Standard Collections*/

    /*Vec<T> (Dynamic array)*/
    /*
    Different ways to crate a Vec
    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![];
    let mut b3 = vec![1i32, 2, 3];//Sufixing 1st value with data type

    let mut b4 = vec![1, 2, 3];
    let mut b5: Vec<i32> = vec![1, 2, 3];
    let mut b6  = vec![1i32, 2, 3];
    let mut b7 = vec![0; 10]; //Ten zeroes
    */

    let mut a = Vec::new();
    a.push(1);
    a.push(2);

    println!("{:?}", a);
    let idx:usize = 0;
    println!("a[{}] = {}", idx, a[idx]);

    // Option
    match a.get(6) {
        None => {println!("Out of Bound")}
        Some(x) => {println!("{}", x)}
    }

    // Iteration
    for x in &a {
        println!("{}", x);
    }

    // Add, Remove
    a.push(3);
    let l = a.pop();

    println!("{:?}", a);
    println!("{}", l.unwrap_or(0));

    // Iteration over Option
    while let Some(x) = a.pop() {
        println!("{}", x);
    }

    /*VecDeque<T> (Double-ended queue)*/


    // LinkedList<T> (Doubly linked list)



    // BinaryHeap<T> (Max heap)


    /*HashMap<K,V> (Dictionary)*/
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("rectangle"), 4);
    shapes.insert(String::from("pentagon"), 5);

    println!("square has {} sides", shapes["square".into()]);

    for (k,v) in &shapes {
        println!("{} has {} sides", k, v);
    }

    // overwrite
    shapes.insert("square".into(), 3);

    // insert if not found
    shapes.entry("circle".into()).or_insert(1);

    {
        let actual = shapes.entry("circle".into()).or_insert(1);
        *actual = 0;
    }

    println!("{:?}", shapes);

    // BTreeMap<K,V> (Sorted dictionary)


    // HashSet<T> (Hashtable) (Unordered set of Unique values)
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);
    greeks.insert("delta"); /*nothing happens*/

    let added = greeks.insert("vega");
    println!("added vega = {}", added);
    println!("{:?}", greeks);
    println!("greeks contains kappa = {}", greeks.contains("kappa"));

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _6_8: HashSet<_> = (6..=8).collect();

    // Check if subset
    println!("Is {:?} a subset of {:?} = {}", _6_8, _6_10, _6_8.is_subset(&_6_10));
    // Check if disjoint i.e. no common elements
    println!("Is {:?} and {:?} are disjoint = {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // Union & Intersection
    println!("{:?} UNION {:?} = {:?}", _1_5, _6_10, _1_5.union(&_6_10));
    println!("{:?} INTERSECTION {:?} = {:?}", _6_8, _6_10, _6_8.intersection(&_6_10));

    // Difference (items in the first set but not in the second) &
    // Symmetric difference (in the union but not in the intersection)
    println!("{:?} DIFFERENCE {:?} = {:?}", _6_10, _6_8, _6_10.difference(&_6_8));
    println!("{:?} SYMDIF {:?} = {:?}", _6_8, _6_10, _6_8.symmetric_difference(&_6_10));

    /*Iterators*/
    let mut v1 = vec![1i32, 2, 3];
    for x in &v1 {
        print!("{}", x);
    }
    println!();
    for x in v1.iter() {
        print!("{}", x);
    }
    println!();

    for x in v1.iter_mut() {
        *x *= 2;
    }

    // print in reverse
    for x in v1.iter().rev() {
        print!("{}", x);
    }
    println!();

    println!("{:?}", v1);

    // into_iter()
    let mut v2 = vec![1i32, 2, 3];
    v2.extend(v1); /* behind the scene extend() uses into_iter() to to transform v1 into an iterator*/
    //println!("{:?}", v1); /*v1 cannot be used anymore*/
    println!("{:?}", v2);



    // BTreeSet<T> (Sorted set)




}