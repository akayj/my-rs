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

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

pub fn make_adder() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}

// 还可以使用 `impl Trait` 返回使用 `map` 或 `filter` 闭包的迭代器
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
