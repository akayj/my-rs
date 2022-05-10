use std::convert::From;

#[derive(Debug)]
struct Number {
    #[allow(dead_code)]
    value: i32,
}

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

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

pub fn make_adder() {
    let y = 12;
    let plus_one = make_adder_function(y);
    // assert_eq!(plus_one(2), 3);
    assert_eq!(plus_one(2), 14);
}

// 还可以使用 `impl Trait` 返回使用 `map` 或 `filter` 闭包的迭代器
fn double_positives<'a>(numbers: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
