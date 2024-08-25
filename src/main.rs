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

fn main() {
    let text = "In the beginning God created the heaven and the earth.
	And the earth was without form, and void; and darkness [was] upon the face of the deep. And the Spirit of God moved upon the face of the waters.
	And God said, Let there be light: and there was light.
	And God saw the light, that [it] [was] good: and God divided the light from the darkness.
	And God called the light Day, and the darkness he called Night. And the evening and the morning were the first day.
	And God said, Let there be a firmament in the midst of the waters, and let it divide the waters from the waters.
	And God made the firmament, and divided the waters which [were] under the firmament from the waters which [were] above the firmament: and it was so.
	And God called the firmament Heaven. And the evening and the morning were the second day.
	And God said, Let the waters under the heaven be gathered together unto one place, and let the dry [land] appear: and it was so.
    And God called the dry [land] Earth; and the gathering together of the waters called he Seas: and God saw that [it] [was] good.
    And God said, Let the earth bring forth grass, the herb yielding seed, [and] the fruit tree yielding fruit after his kind, whose seed [is] in itself, upon the earth: and it was so.
    And the earth brought forth grass, [and] herb yielding seed after his kind, and the tree yielding fruit, whose seed [was] in itself, after his kind: and God saw that [it] [was] good.
    And the evening and the morning were the third day.
    And God said, Let there be lights in the firmament of the heaven to divide the day from the night; and let them be for signs, and for seasons, and for days, and years:
    And let them be for lights in the firmament of the heaven to give light upon the earth: and it was so.
    And God made two great lights; the greater light to rule the day, and the lesser light to rule the night: [he] [made] the stars also.
    And God set them in the firmament of the heaven to give light upon the earth,
    And to rule over the day and over the night, and to divide the light from the darkness: and God saw that [it] [was] good.";

    let chain = train(text);
    let sentence = generate(&chain, 100);

    println!("{sentence}");
}
