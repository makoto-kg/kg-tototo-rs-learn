struct Rect { width: u32, height: u32 }

trait Printable { fn print(&self); }
impl Printable for Rect {
    fn print(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }
}

fn main() {
    let r = Rect { width: 200, height: 300 };
    r.print();
}