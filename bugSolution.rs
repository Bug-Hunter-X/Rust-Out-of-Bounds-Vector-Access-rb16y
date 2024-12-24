fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let idx = 1;

    // Safe way to access vector elements using get()
    match vec.get(idx) {
        Some(val) => println!("Value at index {}: {}", idx, val),
        None => println!("Index {} is out of bounds", idx),
    }

    //Alternative using if let
    if let Some(val) = vec.get(idx) {
        println!("Value at index {}: {}", idx, val);
    } else {
        println!("Index {} is out of bounds", idx);
    }
} 