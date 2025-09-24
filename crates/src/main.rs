use chrono::{Local, Utc};

#[derive(Copy, Clone)]

struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height;
    }
}

fn main() {
    // let r1 = Rect {
    //     width: 10,
    //     height: 20,
    // };

    // let r2 = Rect {
    //     width: 3.9,
    //     height: 7.5,
    // }

    // println!("{}", r1.area());
    // println!("{}", r2.area())

    let utc_time = Utc::now();
    println!("{}", utc_time)
}
