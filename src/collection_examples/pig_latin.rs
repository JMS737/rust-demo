pub fn from(text: &str) -> String {
    let mut new_text = String::new();

    for word in text.split_whitespace() {
        new_text = new_text + &convert_word(word) + " ";
    }

    new_text
}

fn convert_word(word: &str) -> String {
    match &word[0..1] {
        "a" | "e" | "i" | "o" | "u" => convert_vowel(word),
        _ => convert_other(word),
    }
}

fn convert_vowel(word: &str) -> String {
    format!("{}-hay", word)
}

fn convert_other(word: &str) -> String {
    format!("{}-{}ay", &word[1..], &word[0..1])
}
