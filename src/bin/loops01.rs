fn exec01() {
    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("{}", n);
    }
}

fn exec02() {
    for i in 0..10 {
        println!("{}", i);
    }
}

fn exec03() {
    let mut n = 0;
    loop {
        n += 1;
        println!("{}", n);
        if n == 10 {
            break;
        }
    }
}

pub fn main() {
    exec01();
    exec02();
    exec03();
}