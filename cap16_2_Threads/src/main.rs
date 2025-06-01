use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Thread secundaria, value of x is: {:?}", v);
    });
    // o valor de x é movido para dentro da thread, se tentarmos dropar com
    // drop(v) ou println!("{:?}", v), receberemos um erro de ownership, pois a
    // propriedade foi movida para thread secundaria (handle)

    // O valor `v` é movido para dentro da thread secundária pelo `move` no closure.
    // Depois desse movimento, `v` não pode mais ser usado na thread principal — tentar usar `v` na main dará erro de ownership.
    // Rust previne que a thread principal use `v` depois do move, evitando condições de corrida relacionadas a uso pós-move.
    // A thread principal espera a thread secundária terminar com `handle.join()` antes de continuar.

    handle.join().unwrap();
    // Pausa a execução da thread main nesse ponto e espera a thread handle terminar o processo
}