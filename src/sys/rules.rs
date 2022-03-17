// This function takes ownership of a box and destroy it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

pub fn battle() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value
        // is borrowed latter in scope.
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}

// destroy box that point to i32 in heap.
fn destroy_box(c: Box<i32>) {
    println!("Destroying box that contains {}", c);
}

pub fn moves() {
    let x = 5u32;

    // *Copy* `x` to `y` - no resource are moved.
    let y = x;

    println!("x is {}, y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer.
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    let b = a;
    // `a` can not no longer access the data.
    // println!("a contains: {}", a);

    destroy_box(b);
}
