fn main() {
    let apple = "apple";
    let first = "first";

    println!("word {} is {}", apple, to_pig_latin(apple));
    println!("word {} is {}", first, to_pig_latin(first));
}

fn to_pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::from("");
    }

    let mut it = word.chars();
    let first_char = it.next().unwrap();

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", it.as_str(), first_char),
    }
}
