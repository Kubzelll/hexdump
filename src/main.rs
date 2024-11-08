use std::env;
use std::fs;

fn read_file_content(filename: &str) -> Vec<u8>{
    fs::read(filename).unwrap()
}
fn chunk_hex(input: String) -> Vec<String>{
    input
        .as_bytes()
        .chunks(2)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified");
        return;
    }
    let hex_value = chunk_hex(hex::encode(read_file_content(&args[1])));
    println!("{:?}", hex_value);
    

}
