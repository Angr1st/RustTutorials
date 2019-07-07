fn main() {
    let sentence = "Off to do the piglatin thing!";
    let mut iter = sentence.split_whitespace();

    println!();

}

enum WordKind<'a> {
    Vocal(&'a str),
    Consonant(&'a str)
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

fn is_vocal(char:char) -> bool {
    char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' || char == 'A' || char == 'E' || char == 'I' || char == 'O' || char == 'U'
}

fn vocal(word:&str) -> String {
    let lastpart = "-hay";
    format!("{}{}",word,lastpart)
}

fn consonants(word:&str) -> String {
    let firstchar = word.chars().nth(0).expect("A Word should at least contain one character!");
    let rest = get_rest(word);
    let lastpart = "ay";
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