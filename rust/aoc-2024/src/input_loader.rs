pub fn load() -> String {
    let path = std::env::args().nth(1).expect("No input file provided");
    std::fs::read_to_string(path).expect("Failed to read input file")
}
