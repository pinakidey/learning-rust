use std::thread;
use std::time;

pub fn run() {
    // Spawn a thread
    let t = thread::spawn(move || {
        for _ in 1..10 {
            print!("1");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 {
        print!("0");
        thread::sleep(time::Duration::from_millis(300));
    }


    t.join().unwrap();
}