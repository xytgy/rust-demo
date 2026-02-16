fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    let (x, y, z) = tup;
    // y 等于 6.4
    println!("x = {}, y = {}, z = {}", x, y, z);
}
