use std::collections::HashMap;
fn main() {
    let contents = std::fs::read("faust.txt").unwrap(); // little edian
    let contents: Vec<u16> = contents.chunks(2).map(|bytes| {
        let [first, second] = [bytes[0], bytes[1]];
        (second as u16) << 8 | (first as u16)
    }).collect();
    let contents = String::from_utf16(&contents[..]).unwrap();
    let hash = process(&contents);
    for sen in &hash["The"] {
        println!("{}", sen);
    }
}

fn process<'a>(string: &'a str)-> HashMap<&'a str, Vec<&'a str>> {
    let mut result:HashMap<&str, Vec<&str>> = HashMap::new();
    for line in string.lines() {
        for sentence in line.split_terminator('.'){
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    result.entry(word).or_default().push(sentence);
                }
            }
        }
    }
    result
}