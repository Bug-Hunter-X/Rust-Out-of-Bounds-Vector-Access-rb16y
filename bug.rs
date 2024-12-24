fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let idx = 1;
    // This will panic if idx is out of bounds
    let val = vec[idx];
    println!("Value at index {}: {}", idx, val);
}