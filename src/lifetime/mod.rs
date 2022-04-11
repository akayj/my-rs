pub mod bounds;
pub mod conv;
pub mod traits;

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}, x + y = {}", x, y, x + y);
}

// fn failed_borrow<'a>() {
//     let _x = 12;
//     let y: &'a i32 = &_x;
// }

// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn invalid_output<'a>() -> String {
    String::from("foo")
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn life_check() {
    let s = invalid_output();
    println!("invalid output is: {}", s);

    print_refs(&1, &2);

    let mut x = 12_i32;
    add_one(&mut x);
    println!("x is {}", x);

    let y = pass_x(&x, &2);
    println!("y is {}", y);

    let x = 18;
    let y = 15;

    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("reference: {:?}", reference);
    println!("number: {:?}", number);

    conv::main();
}
