mod json;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let json_file = std::fs::read_to_string(&args[1]).unwrap();

    let value = json::parse_from_string(json_file);

    println!("parsed value :");
    println!("{:?}", value);
}
