fn main() {
    println!("Enter the <sentence>,<word-to-find>");

    let mut args = String::new();
    std::io::stdin().read_line(&mut args).unwrap();

    let args = args.trim();
    println!("args {:?}", args);

    let mut parts = args.split(",");
    let sentence = parts.next().unwrap();
    let word = parts.next().unwrap();

    println!("sentence {:?}", sentence);
    println!("word {:?}", word);
    if sentence.contains(word) {
        println!("Word is present in the sentence");
    }
    else {
        println!("Word is not present in the sentence");
    }

}
