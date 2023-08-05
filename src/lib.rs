use clap::Parser;
use std::{path, fs::File, io::{BufReader, BufRead}, process };

#[derive(Parser)]
pub struct Cli {
    path: path::PathBuf
}

struct FileDetails {
    lines: i32,
    words: usize,
    bytes: usize
}

pub fn run(args: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let details = read_file(&args.path)?;

    println!("{} lines, {} words, {} bytes", details.lines, details.words, details.bytes);

    Ok(())
}

fn read_file(path: &path::PathBuf) -> Result<FileDetails, Box<dyn std::error::Error>> {

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    let mut file_details = FileDetails {
        lines: -1,
        words: 0,
        bytes: 0
    };

    while reader.read_line(&mut content).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    }) > 0 {
        let _ = content.trim();

        file_details.lines += 1;

        let no_of_words: Vec<&str> = content.split(" ").take_while(|word| !word.is_empty()).collect();

        file_details.words += no_of_words.len();

        file_details.bytes += content.len();

        content.clear()
    }

    Ok(file_details)
}