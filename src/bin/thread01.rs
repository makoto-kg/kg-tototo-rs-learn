use std::thread;
use std::time::Duration;

fn main() {
    let th = thread::spawn(|| {
        for _i in 1..10 {
            println!("A");
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finished");
}