fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(1..10);
    println!("The value of x is {}", x);
}
