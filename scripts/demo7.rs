fn main() {
    let x: u8 = 0b1010; // 10
    let y: u8 = 0b1100; // 12

    println!("x & y = {:b}", x & y);
    println!("x | y = {:b}", x | y);
    println!("x ^ y = {:b}", x ^ y);
    println!("!x = {:b}", !x);
    println!("x << 1 = {:b}", x << 1);
    println!("x >> 1 = {:b}", x >> 1);
}