fn main() {
    let mut n = 5;

    n += 3;
    println!("n += 3 -> {}", n);

    n *= 2;
    println!("n *= 2 -> {}", n);

    n >>= 1;
    println!("n >>= 1 -> {}", n);
}