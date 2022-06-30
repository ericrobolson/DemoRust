extern "C" {
    fn bar_function(x: i32) -> i32;
}

pub fn call() -> i32 {
    unsafe { bar_function(42) }
}

fn main() {
    println!("{:?}", call())
}
