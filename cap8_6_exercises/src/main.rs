use std::collections::HashMap;

fn main() {
    let word = "hello apple first";
    let word = latin_pig(&word);
    println!("{word}")
}

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    let mid = len / 2;

    if len % 2 == 0 {
        (arr[mid] as f64 + arr[mid - 1] as f64) / 2.0
    } else {
        arr[mid] as f64
    }
}

fn mode(arr: &[i32]) -> i32 {
    let mut hash = HashMap::new();

    for num in arr {
        let value = hash.entry(*num).or_insert(0);
        *value += 1;
    };

    let mut high = arr[0];

    for (key, value) in &hash {
        if hash.get(&high).unwrap() < value {
            high = *key;
        }
    }

    high
}

fn latin_pig(string: &str) -> String {
    let mut phrase_latin = String::new();

    for word in string.split_whitespace() {
        let mut chars = word.chars();

        if let Some(chr) = chars.next() {
            if matches!(chr, 'a' | 'e' | 'i' | 'o' | 'u') {
                phrase_latin.push_str(&format!("{word}-hay "));
            } else {
                // collect() consome o restante do iterador, transformando no type
                let add: String = chars.collect();
                phrase_latin.push_str(&format!("{add}-{chr}ay "))
            }
        }
    }

    phrase_latin.trim_end().to_string()
}