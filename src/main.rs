fn main() { 
    println!("Off to do the piglatin thing!");
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