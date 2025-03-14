use rand::seq::SliceRandom;

fn main() {
    let sentence = "Hey Doug, I'm worried that our friendship is only parasocial, can you acknowledge this message so that it isn't anymore?";
    
    let nouns = vec!["Doug", "friendship", "message"];  // List of nouns in the sentence

    let mut rng = rand::thread_rng();
    let random_noun = nouns.choose(&mut rng).unwrap();  // Select a random noun

    let modified_sentence = sentence.replace(random_noun, "bibleotecha");

    println!("Original Sentence: {}", sentence);
    println!("Modified Sentence: {}", modified_sentence);
}

