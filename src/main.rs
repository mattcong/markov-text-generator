use clap::Parser;
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn train(text: &str) -> HashMap<(String, String), Vec<String>> {
    let mut chain = HashMap::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for pair in words.windows(3) {
        chain
            .entry((pair[0].to_string(), pair[1].to_string()))
            .or_insert_with(Vec::new)
            .push(pair[2].to_string());
    }

    chain
}

fn generate(chain: &HashMap<(String, String), Vec<String>>, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let keys: Vec<&(String, String)> = chain.keys().collect();
    let mut sentence: Vec<String> = vec![];

    if let Some(&start_pair) = keys.choose(&mut rng) {
        sentence.push(start_pair.0.clone());
        sentence.push(start_pair.1.clone());

        while sentence.len() < length {
            let pair = (
                sentence[sentence.len() - 2].clone(),
                sentence[sentence.len() - 1].clone(),
            );
            if let Some(next_words) = chain.get(&pair) {
                if let Some(next_word) = next_words.choose(&mut rng) {
                    sentence.push(next_word.clone());
                } else {
                    break;
                }
            }
        }
    }

    sentence.join(" ")
}

#[derive(Parser)]
struct Cli {
    text: String,
    length: usize,
}

fn main() {
    let cli = Cli::parse();

    let chain = train(&cli.text);
    let sentence = generate(&chain, cli.length);

    println!("{}", sentence);
}
