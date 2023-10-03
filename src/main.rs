use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn c(file_name: &str) -> u64 {
    let metadata = fs::metadata(file_name).expect("Unable to open file");
    metadata.len()
}

fn l(file_name: &str) -> i64 {
    let buffer = BufReader::new(File::open(file_name).unwrap());

    let mut lines = 0;
    for _ in buffer.lines() {
        lines += 1;
    }
    lines
}

fn w(file_name: &str) -> usize {
    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s.trim_end().chars().count()
}

fn main() {
    // We collect the params
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-c" => {
            let file_name = &args[2];
            println!("{} {}", c(file_name), file_name);
        }
        "-l" => {
            let file_name = &args[2];

            println!("{} {}", l(file_name), file_name);
        }
        "-w" => {
            let file_name = &args[2];
            println!("There are {} characters in the file", w(file_name));
        }
        _ => {
            let file_name = &args[2];
            println!(
                "{} {} {} {}",
                c(file_name),
                l(file_name),
                w(file_name),
                file_name
            );
        }
    }
}
