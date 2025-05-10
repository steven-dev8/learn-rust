use std::collections::HashMap;

fn main() {
    let palavras = vec!["bat", "tab", "tab", "tap", "pat", "cat"];
    let group = anagram_group(palavras);
    println!("{group:?}")
}

fn anagram_group(arr: Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut table: HashMap<String, Vec<String>> = HashMap::new();

    for word in arr {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        let key: String = chars.iter().collect();
        
        table.entry(key).or_default().push(word.to_string());
    }

    table
}