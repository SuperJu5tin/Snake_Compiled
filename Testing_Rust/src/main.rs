fn main() {
    let mut a = vec![1, 2];
    let c: Vec<_> = a.splice(0..0, [1]).collect();
    let b = vec![1, 0];
    println!("{}", a[1])
}