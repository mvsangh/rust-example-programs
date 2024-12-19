use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    FindInSentence { sentence: String, word: String },
    FindInFile { path: PathBuf, pattern: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::FindInSentence { sentence, word } => {
            find_words_in_sentence(&sentence, &word);
        }
        Commands::FindInFile { path, pattern } => {
            sleep(Duration::from_secs(2));
            find_pattern_from_file(path, pattern);
        }
    }
}

fn find_words_in_sentence(sentence: &str, word: &str) {
    println!("Sentence: {:?}", sentence);
    println!("Word: {:?}", word);

    if sentence.contains(word) {
        println!("Word is present in the sentence\n");
    } else {
        println!("Word is not present in the sentence\n");
    }
}

fn find_pattern_from_file(filename: PathBuf, pattern: String) {
    let file_content = std::fs::read_to_string(filename).unwrap();

    for line in file_content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
}
