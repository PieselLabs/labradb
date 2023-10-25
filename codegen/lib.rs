use std::time::Instant;

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    let now = Instant::now();

    let elapsed_time = now.elapsed();

    println!("Hello world: {}!", elapsed_time.as_secs());

    a + b
}
