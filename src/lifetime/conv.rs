fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// <'a: 'b, 'b> 生命周期 'a 至少和 'b 一样长
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

pub fn main() {
    let first = 2; // 较长的生命周期

    {
        let second = 3; // 较短的生命周期

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
