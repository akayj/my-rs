use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn my_from() {
    let num = Number::from(30);
    log::debug!("My number is {:?}", num);
}
