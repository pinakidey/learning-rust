use std::mem;
pub fn run() {
    // struct
    struct Point {
        x: f64,
        y: f64
    }
    struct Line {
        start: Point,
        end: Point
    }
    fn structures() {
        let p1 = Point{x: 1.0, y: 0.0};
        println!("{} {}", p1.x, p1.y);

        let p2 = Point{x: 5.0, y: 10.0};
        let _line1 = Line{start: p1, end: p2};
    }

    // enum
    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8,u8,u8), //tuple
        Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} //struct
    }

    fn enums () {
        let c:Color = Color::Cmyk{ cyan: 0, magenta: 0, yellow: 0, black:255};

        match c {
            Color::Red=> {println!("R")}
            Color::Green => {println!("G")}
            Color::Blue => {println!("B")}
            Color::RgbColor(0,0,0) => {println!("Black")}
            Color::Cmyk{black:255,..} => {println!("Black")}
            Color::RgbColor(r,g,b) => {println!("RGB({},{},{})", r,g,b)}
            _ => ()
        }
    }

    enums();

    // union

    union IntOrFloat {
        i: i32,
        f: f32
    }

    fn union() {
        let mut iof = IntOrFloat{i: 123};
        iof.i = 234;
        let value = unsafe {iof.i};
        // since we don't know at runtime what would be the type of value stored in IntOrFloat
        // we have to use an unsafe block
        println!("{}", value);
    }
    union();

    fn process_value(iof:IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i } => {println!("{}", i)}
                IntOrFloat { f } => {println!("{}", f)} // does re-interpret cast
            }
        }
    }

    process_value(IntOrFloat{i: 123});
    process_value(IntOrFloat{f: 3.14}); //careful


    // Option<T> -> Some(v) | None
    let x=3.0;
    let y=1.0;
    let result = if y!=0.0 {Some(x/y)} else {None};

    match result {
        None => {println!("Undefined")}
        Some(z) => {println!("{}", z)}
    }

    // if let, while let
    if let Some(z) = result {
        println!("result {}", z)
    }

    // Array
    let mut a:[i32; 5] = [1,2,3,4,5];
    a[0] = 0;
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("{:?}", a);
    if a!= [1,2,3,4,5] {println!("Not same")}

    let b = [1u16; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    println!("b takes {} bytes", mem::size_of_val(&b));

    // 2D Array (Matrix)
    let mtx:[[f32;3];2] = [
        [1.0, 0.0, 0.1],
        [1.2, 2.3, 3.4]
    ];
    println!("{:?}", mtx);

    // Slice
    fn use_slices(slice: &mut [i32]) {
        slice[0] = 123;
        println!("{:?}", slice);
    }
    fn slice() {
        let mut data = [1,2,3,4,5];
        use_slices(&mut data[1..4]); /*not including 4th element*/
        use_slices(&mut data);
    }
    slice();

    // Tuple (can use different types)
    fn tuple() {
        let sp = sum_and_product(3, 4);
        // destructuring
        let (a,b) = sp;
        println!("sp={:?} sum={}, product={}", sp, a, b);
        // tuple of tuple
        let sp2 = sum_and_product(4, 5);
        let combined = (sp, sp2);
        println!("last element = {}", combined.1.1);
        // destructuring
        let ((a,b),(c,d)) = combined;
        println!("a={} b={} c={} d={}", a,b,c,d);

        // tuple of single element
        let t2 = (12, );
        println!("{}", t2.0);
    }
    fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
        return (x+y, x*y);
    }
    tuple();

    // Pattern Matching
    for x in 0..13 {
        println!("{} means {} elements", x, count(x));
    }
    fn count(x:i32) -> &'static str {
        match x {
            0 => "no",
            1 | 2 => "one or two",
            12 => "a dozen of",
            _z @10...13 => "lots of",
            _ if x%2==0 => "even number of",
            _ => "a few"
        }
    }
    // Match on tuple
    let point = (3,4);
    match point {
        (0,0) => println!("Origin"),
        (a,b) => println!("Point ({},{})", a,b)
    }

    // Generics
    struct point<T> {
        x: T,
        y: T
    }
    struct line<T,V> {
        start: point<T>,
        end: point<V>
    }
    fn generics() {
        let a = point {x: 0, y: 1};
        let b = point {x: 1.0, y: 3f64};
        let l = line {start: a, end: b};

        println!("{:?}", l.start.y);
    }

    generics();

}