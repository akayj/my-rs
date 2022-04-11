use futures::executor::block_on;

async fn hello_world() {
    println!("async Hello world");
}

pub fn main() {
    let future = hello_world();
    block_on(future);
}
