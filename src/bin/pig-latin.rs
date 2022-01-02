fn main() {
    let text = "first apple pig latin";
    println!("{}", text);
    for word in text.split_whitespace() {
        println!("{}", make_pig_latin(word));
    }
}

fn is_vowel(ch: char) -> bool {
    "euioa".contains(ch)
}

fn make_pig_latin(word: &str) -> String {
    let mut s = String::from(word);
    let ch = s.chars().next().unwrap();

    if is_vowel(ch) {
        s.push_str("hay");
    }
    else {
        s.remove(0);
        s.push(ch);
        s.push_str("ay");
    }

    s
}
