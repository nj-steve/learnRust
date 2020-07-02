fn main() {
    for node in 0..=10{
        let hash = if node > 3 {
            node * 2
        } else {
            node -1
        };
        println!("hhhhh = {}",hash);
    }
    println!("Hello, world!");
}
