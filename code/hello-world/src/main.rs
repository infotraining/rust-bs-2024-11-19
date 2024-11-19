fn main() {
    println!("Hello, world!");

    let x: u8 = 255;
    let y = x.wrapping_add(1);

    println!("x: {}", x);
    println!("y: {}", y);

    let z = x.checked_add(1);
    match z {
        Some(value) => println!("z: {}", value),
        None => println!("overflow"),
    }

    let a = x.saturating_add(1);
    println!("a: {}", a);
}
