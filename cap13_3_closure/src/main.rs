#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let mut list = [
        Rectangle { width: 7, height:10 },
        Rectangle { width: 11, height:9 },
        Rectangle { width: 1, height: 3 },
    ];

    list.sort_by_key(
        |x| x.width
    );

    println!("{list:#?}");

    let mut store = Vec::new();
    let value = String::from("Closure here");

    list.sort_by_key(
        |x| {
            // store.push(value); Da erro, pois value perde a posse da string na primeira iteração
            store.push(x.width); // store.push(x.width); daria erro, pois sort_by_key é FnMut e não FnOnce
            x.width
        }
    );

    let mut list2 = [
        Rectangle { width: 7, height:10 },
        Rectangle { width: 11, height:9 },
        Rectangle { width: 1, height: 3 },
    ];

    let mut num_sort_operations = 0;
    list2.sort_by_key(
        |x| {
            num_sort_operations += 1;
            x.height
        }
    );

    println!("{list2:#?}, sorted in {num_sort_operations} operations");
}
