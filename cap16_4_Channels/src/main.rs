use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Salve,"),
            String::from("Steven."),
            String::from("Tudo bem"),
            String::from("?")
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            // Envia item os valores, item por item que vão ser recebidos pelo rx
            thread::sleep(Duration::from_secs(1));
        }
    });

    for val in rx {
        println!("{}", val);
    }
    // O rx (Receptor) implementa a trait Iterator e, internamente, chama recv().
    // Portanto, os itens podem ser iterados à medida que são recebidos,
    // e a thread fica bloqueada até o tx ser destruído (canal fechado).

    // Forma equivalente:
    // while let Ok(val) = rx.recv() {
    //     println!("{val}");
    // }
}
