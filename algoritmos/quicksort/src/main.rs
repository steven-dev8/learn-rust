fn main() {
    let arr = vec![9, 8, 2, 3, 5, 1, 2, 2];
    let teste = quicksort(arr);
    println!("{:?}", teste);
}

fn quicksort(arr: Vec<i64>) -> Vec<i64> {
    if arr.len() <= 1 {
        return arr;
    };

    let pivo = arr[arr.len() / 2];
    let mut left: Vec<i64> = vec![];
    let mut mid: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    
    for item in &arr {
        if *item < pivo {left.push(*item)}
        else if *item == pivo {mid.push(*item)}
        else {right.push(*item)}
    }

    let left = quicksort(left);
    let right = quicksort(right);

    let mut sorted: Vec<i64> = Vec::new();
    sorted.extend(left);
    sorted.extend(mid);
    sorted.extend(right);

    sorted
}