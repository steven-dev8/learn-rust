fn main() {
    let string = "Salve rapaziada";
    println!("{}", leet_speak(&string));
}

fn leet_speak(string: &str) -> String {
    let mut phrase = String::new();

    for word in string.split_whitespace() {
        let mut chars = word.chars();

        while let Some(chr) = chars.next() {
            match chr.to_lowercase().next().unwrap() {
                'a' => phrase.push('4'),
                'e' => phrase.push('3'),
                'i' => phrase.push('1'),
                'o' => phrase.push('0'),
                's' => phrase.push('5'),
                other => phrase.push(chr),
            }
        }
        phrase.push(' ');
    }

    phrase.trim_end().to_string()
}
