use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::num::ParseIntError;
use notify::Config;
use notify::{event::Event, EventKind, RecommendedWatcher, Watcher, RecursiveMode};
struct Cleanup;
impl Cleanup {
    fn cleanup_function() {
        destroy();
    }
}
impl Drop for Cleanup {
    fn drop(&mut self) {
        Self::cleanup_function();
    }
}

const ENDING: &str = ".txt";
fn main() {
    let _cleanup = Cleanup;
    
    let file_bytes = fs::read(Path::new(&("to-read/to-read".to_owned()+&ENDING))).expect("failed to read file in preparation of hex conversion");

    let t = Instant::now();

    let hex_array: Vec<String> = file_bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    let path_to_hex_container = Path::new("hex.txt");
    {
        let hex_to_write = hex_array.join(" ");
        let hex_container = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path_to_hex_container)
            .expect("failed to open hex container file");
        let mut hex_file_writer = BufWriter::new(&hex_container);

        hex_file_writer.write_all(hex_to_write.as_bytes()).expect("failed to write bytes");
        hex_file_writer.flush().expect("failed while writing hex to file");
        
    }
    println!("time taken to retrieve and place Hex: {} miliseconds", t.elapsed().as_millis());

    let (tx, rx) = channel();
    let config = Config::default();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, config).expect("failed to generate file watcher");
    watcher.watch(path_to_hex_container, RecursiveMode::NonRecursive).expect("failed to watch file");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(Event { kind: EventKind::Modify(_), paths, attrs: _ }) => {
                    println!("File has been modified!");
                    if let Some(path) = paths.first() {
                        let hex_file_contents = fs::read_to_string(path).expect("failed to read file");
                        let separate_hex_values: Vec<&str> = hex_file_contents.split_ascii_whitespace().collect();
                        if separate_hex_values != hex_array {
                        give_results_on_save(hex_file_contents);
                        }
                    }
                }
                Ok(Event { kind: EventKind::Remove(_), paths: _, attrs: _ }) => {
                    println!("File has been removed by user");
                }
                _ => {}
            },
            Err(e) => eprintln!("Error watching file: {:?}", e),
        }
    }
}

fn give_results_on_save(hex_file_contents: String) {
    let bytes_to_write = hex_to_bytes(&hex_file_contents).expect("failed turning hex to u8");
    let mut result_file = OpenOptions::new().read(true)
    .write(true)
    .create(true)
    .truncate(true)
    .open(&("result".to_owned()+&ENDING))
    .unwrap();
    result_file.write_all(&bytes_to_write).expect("failed to write to file");
    result_file.flush().expect("failed while writing to file");
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let hex = hex.trim().replace(" ", "");

    let hex = if hex.len() % 2 == 0 { hex } else { 
        panic!("INVALID HEX INSERTED")
    };

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16))
        .collect()
}
fn destroy() {
    let path_to_hex_container = Path::new("hex.txt");
    fs::remove_file(path_to_hex_container).expect("failed to delete file");
}
