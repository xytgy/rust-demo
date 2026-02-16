fn main() {
    let x = 5;
    let y = x as f64;

    for i in 1..4 {
        print!("{} ", i);
    }
    println!();

    for i in 1..=3 {
        print!("{} ", i);
    }
    println!();

    let a = 10;
    let b = &a;
    println!("*b = {}", *b);
}