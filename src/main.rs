use std::env;
use std::fs;

//TODO: Add handling if file doesn't exist
fn read_file_content(filename: &str) -> Vec<u8>{
    fs::read(filename).unwrap()
}
fn chunk_hex(input: String) -> Vec<String>{
    input
        .as_bytes()
        .chunks(4)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}
fn parse_hex(input_vec: Vec<String>) -> Vec<Vec<String>>{
    let chunked_vec: Vec<Vec<String>> = input_vec
        .chunks(8)
        .map(|chunk| chunk.to_vec())
        .collect();


    chunked_vec
}
fn final_format(input: Vec<Vec<String>>) -> String{
    let mut formatted = String::new();
    let mut hex_offset: i32 = 0;
    let rows_num = input.len();
    for i in 0..rows_num{
        let row_size = input[i].len();
        formatted += &*format!("{}\t", hex::encode(hex_offset.to_be_bytes()));
        for x in 0..row_size{
            formatted += &*format!("{} ", input[i][x]);
        }
        hex_offset += 16;
        formatted += "\n";
    }
    formatted
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified");
        return;
    }
    let file_content = read_file_content(&args[1]);
    let hex_value = chunk_hex(hex::encode(file_content));
    let parsed_hex = parse_hex(hex_value);
    println!("{}", final_format(parsed_hex));
    

}
