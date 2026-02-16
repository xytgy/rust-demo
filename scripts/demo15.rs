fn main() {
    let a = 10;
    let b = 20;
    let c = add(a, b);
    println!("c = {}", c);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}