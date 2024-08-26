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
    let mut generated_text: Vec<String> = vec![];

    if let Some(&start_pair) = keys.choose(&mut rng) {
        generated_text.push(start_pair.0.clone());
        generated_text.push(start_pair.1.clone());

        while generated_text.len() < length {
            let pair = (
                generated_text[generated_text.len() - 2].clone(),
                generated_text[generated_text.len() - 1].clone(),
            );
            if let Some(next_words) = chain.get(&pair) {
                if let Some(next_word) = next_words.choose(&mut rng) {
                    generated_text.push(next_word.clone());
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    generated_text.join(" ")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[derive(Parser)]
struct Cli {
    text: String,
    length: usize,
}

fn main() {
    let cli = Cli::parse();
    let max_text_length = cli.text.split_whitespace().count() * 2;

    if cli.length > max_text_length {
        eprintln!(
            "Warning: Text length ({}) is too long for the input provided. \
            Generating shorter text instead.",
            cli.length
        );
    }

    let chain = train(&cli.text);
    let generated_text = generate(&chain, cli.length);

    println!("Generated text: {}", capitalize(&generated_text));
}
