fn main() {
    let filename: String = std::env::args().nth(1).unwrap();
    let message: String = std::fs::read_to_string(&filename).unwrap();
    let bc = message.len(); 
    let wc: usize = message.split_whitespace().count();
    let lc: usize = message.split('\n').count() - 1;
    println!("  {}  {} {} {}", lc, wc, bc, filename);
}
