use std::path::Path;

pub fn load(day: u32) -> String {
    let default_path = Path::new("inputs").join(format!("{:02}.txt", day));
    let arg_path = std::env::args().nth(1);
    let path = arg_path
        .as_deref()
        .unwrap_or(default_path.to_str().unwrap());

    std::fs::read_to_string(path).expect("Failed to read input file")
}
