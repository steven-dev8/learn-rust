use std::collections::HashMap;

fn main() {
    let array = [1, 2, 2, 4, 5, 6, 7, 8];

    let mediana = median(&array);
    println!("{mediana}");

    let moda = mode(&array);
    println!("{moda}")
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