use std::thread;
use std::time::Duration;

fn main() {
    let str = String::from("ABC");
    let th = thread::spawn(move || {
        for _i in 1..10 {
            println!("{}", str);
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finished");
}