use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

struct Empty;
struct Null;

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub fn my_from() {
    let num = Number::from(30);
    log::debug!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    log::debug!("My number is {:?}", num);

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
