fn main() {
    let n = 5;
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in 0..n {
        if i % 2 == 0 {
            arr[i] *= 2;
        }
    }

    println!("Modified array: {:?}", arr);
}
