/// The addition program in Rust.

// this will compile
fn add_two<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// this won't
// fn add_two<T: std::ops::Add>(a: T, b: T) -> T {
//     a + b
// }

// this won't either
// fn add_two<T>(a: T, b: t) -> T {
//     a + b
// }

fn main() {
    assert_eq!(add_two(1, 2), 3);
}
