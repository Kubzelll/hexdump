use std::env;
use std::fs;
use std::io::{Error, ErrorKind};


fn read_file_content(filename: &str) -> Result<Vec<u8>, Error> {
    let read = fs::read(filename);
    match read{
        Ok(_) => {
            Ok(read?)
        }
        Err(_) => {
            Err(Error::new(ErrorKind::Other, "Could not read file"))
        }
    }
    
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
            if x == 0 {
                formatted += "| ";
            }
            formatted += &*format!("{} ", input[i][x]);
            if x+1 == row_size{
                formatted += " |";
            }
        }
        hex_offset += 16;
        formatted += "\n";
    }
    formatted
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: No file specified\n");
        return;
    }
    match read_file_content(&args[1]){
        Ok(content) => {
            let hex_value = chunk_hex(hex::encode(content));
            let parsed_hex = parse_hex(hex_value);
            println!("{}", final_format(parsed_hex));
        }
        _ => {
            println!("Error: Could not read file\n");
            return;
        }
    }
    
}
