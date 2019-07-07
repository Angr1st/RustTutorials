fn main() {
    let sentence = "Off to do the piglatin thing!";
    let piglatin = transform_to_piglatin(sentence);

    println!("Oy piglatin: {}",piglatin);

}

enum WordKind<'a> {
    Vocal(&'a str),
    Consonant(&'a str)
}

fn transform_to_piglatin(sentence:&str) -> String {
    let iter = sentence.split_whitespace();
    transform_words(sort_words(iter))
}

fn sort_words(words:std::str::SplitWhitespace) -> Vec<WordKind> {
    let mut vec = Vec::new();
    for word in words {
        if is_vocal(word.chars().nth(0).expect("A word should contain at least one char!")) {
            vec.push(WordKind::Vocal(word));
        }
        else {
            vec.push(WordKind::Consonant(word));
        }
    }
    vec
}

fn transform_words(words:Vec<WordKind>) -> String {
    let mut result = String::new();
    for word in words {
        match word {
            WordKind::Vocal(a) => result.push_str(&vocal(&a)),
            WordKind::Consonant(a) => result.push_str(&consonants(&a))
        }
    }
    result
}

fn is_vocal(char:char) -> bool {
    char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' || char == 'A' || char == 'E' || char == 'I' || char == 'O' || char == 'U'
}

fn vocal(word:&str) -> String {
    let lastpart = "-hay ";
    format!("{}{}",word,lastpart)
}

fn consonants(word:&str) -> String {
    let firstchar = word.chars().nth(0).expect("A Word should at least contain one character!").to_lowercase();
    let rest = get_rest(word);
    let lastpart = "ay ";
    format!("{}-{}{}",rest,firstchar,lastpart)
}

fn get_rest(word:&str) -> String {
    let mut result = String::new();
    let mut iter = word.chars();
    iter.next();
    for char in iter {
        result.push(char);
    }
    result
}