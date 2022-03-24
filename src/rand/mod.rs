extern crate rand;

use rand::prelude::*;

pub fn rand_number() {
    let x: u8 = random();
    println!("{}", x);
}
