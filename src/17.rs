use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::<i32>::new();
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    q.push_back(6);
    q.push(numbers.iter().cloned().filter(|x| x % 2 == 0).next().unwrap());
    println!("{:?}", q);
}
