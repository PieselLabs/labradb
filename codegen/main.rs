#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    println!("Hello world!");

    a + b
}
