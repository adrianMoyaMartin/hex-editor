use core::time;
use std::{fs::{self, File, OpenOptions}, io::{BufReader, BufWriter, Read, Write}, path::Path, thread::sleep};
use std::num::ParseIntError;

const ENDING: &str = ".txt";

fn main() {
    let file_to_read = File::open(Path::new(&("to-read/to-read".to_owned()+&ENDING)))
        .expect("Failed opening file");
    let buf = BufReader::new(file_to_read);
    
    let mut file_bytes: Vec<u8> = vec![];
    for line in buf.bytes() {
        let byte = line.expect("failed to read line");
        file_bytes.push(byte);
    }

    let hex_array = bytes_to_hex(file_bytes);

    let path_to_hex_container = Path::new("hex.txt");
    let hex_container = File::create_new(path_to_hex_container).expect("couldnt create file with hex values");
    let mut hex_file_writer = BufWriter::new(&hex_container);

    for hex in &hex_array {
        let hex_to_write = " ".to_owned()+&hex;
        hex_file_writer.write_all(hex_to_write.as_bytes()).expect("failed to write hex to file");
    }
    
    hex_file_writer.flush().expect("failed while writing hex to file");

    loop {
        let hex_file_contents = fs::read_to_string(path_to_hex_container).expect("failed to read file");
        let separate_hex_values: Vec<&str> = hex_file_contents.split_ascii_whitespace()
        .into_iter()
        .collect();

        if &separate_hex_values != &hex_array {
            give_results_on_save(hex_file_contents, path_to_hex_container);
            break;
        }
        sleep(time::Duration::from_millis(100));
    }
    destroy(path_to_hex_container);
}

fn give_results_on_save(hex_file_contents: String, path_to_hex_container: &Path ) {
    let bytes_to_write = hex_to_bytes(&hex_file_contents, path_to_hex_container).expect("failed turning hex to u8");
    let mut result_file = OpenOptions::new().read(true)
    .write(true)
    .create(true)
    .truncate(true)
    .open(&("result".to_owned()+&ENDING))
    .unwrap();
    result_file.write_all(&bytes_to_write).expect("failed to write to file");
    result_file.flush().expect("failed while writing to file");
}

fn bytes_to_hex(bytes: Vec<u8>) -> Vec<String> {
    let hex_array = bytes.iter()
    .map(|byte| format!("{:02X}", byte))
    .collect();
    hex_array
}

fn hex_to_bytes(hex: &str, path_to_hex_container: &Path) -> Result<Vec<u8>, ParseIntError> {
    let hex = hex.trim().replace(" ", "");

    let hex = if hex.len() % 2 == 0 { hex } else { 
        destroy(path_to_hex_container);
        panic!("INVALID HEX INSERTED")
    };

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16))
        .collect()
}
fn destroy(path_to_hex_container: &Path) {
    fs::remove_file(path_to_hex_container).expect("failed to delete file");
}
