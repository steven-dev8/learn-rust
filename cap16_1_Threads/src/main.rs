use std::thread;
use std::time::Duration;

fn main() {
    // sem o handle, a thread main poderia terminar primeiro sem executar a thread de handle
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Rust thread is couting: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    // o retorno de uma thread::spawn() é JoinHandle<T>

    // chamar um handle.join().unwrap() aqui, pausaria a thread nesse ponto, após o retorno
    // de thread handle, a thread continuaria de onde parou

    for i in 0..5 {
        println!("Rust main is couting: {i}");
        thread::sleep(Duration::from_millis(1));
    };

    handle.join().unwrap()
    // isso obriga a thread main a esperar a thread handle terminar
    // chamar .join() pausa a execução da thread atual,
    // impossibilitando a execução de trabalho ou sair (terminar)
}
