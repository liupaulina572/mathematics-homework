// Return a random number between 1 and 10
fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(1..=10));
}
