fn main() {
    let b = greater_than_one(3);
    println!("Condition was {}", b);
}

fn greater_than_one(x: i32) -> bool {
    if x > 1 {
        true
    } else {
        false
    }
}
