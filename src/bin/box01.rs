struct Point {
    x: u8,
    y: u8
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

pub fn main() {
    let p: Box<Point> = Box::new(Point { x: 10, y: 10});
    println!("{}, {}", p.x, p.y);
}