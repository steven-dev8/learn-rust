use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    // é possível clonar quantos tx (Transmisores) que você quiser, todos vão redirecionar
    // os dados para o mesmo rx (Receptor)
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("Guys"),
            String::from("My name"),
            String::from("is Steven"),
        ];

        for val1 in vals {
            tx1.send(val1).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val2 in vals {
            tx.send(val2).unwrap();
            thread::sleep(Duration::from_millis(500));
        };
    });

    for values in rx {
        println!("Got: {values}");
    }

    // rx implementa a trait Iterator, então você pode iterar sobre ele diretamente.
    // Internamente, ao iterar sobre rx, ele chama recv() repetidamente,
    // fazendo pattern matching com Ok(val). A iteração termina quando o último tx é destruído.
}
