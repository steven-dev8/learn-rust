use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    // Arc é um smart pointer thread-safe que permite compartilhar a posse do dado entre múltiplas threads,
    // usando contagem de referências atômicas para garantir que o dado não seja destruído enquanto houver
    // referências ativas.
    // Mutex protege o dado garantindo exclusão mútua, evitando data races.    

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
           
            *count += 1;
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.join().unwrap();
    };

    println!("Result: {}", *counter.lock().unwrap());
}
