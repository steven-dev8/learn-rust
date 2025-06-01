use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    // mpsc é uma biblioteca padrão que fornece "channels" usado para compartilhar
    // dados entre threads, o método mpsc::channel() retorna uma tupla contendo um
    // tx (transmissor) e rx (receptor)

    // Uma channel é usado como um canal de fluxo de dados entre duas threads (ou mais)

    thread::spawn(move || {
        let val = String::from("Olá Mundo");
        tx.send(val).unwrap();
        // movemos o tx (transmissor) para a thread sec, pois ela vai ser a fonte de
        // transmissão entre os dados que serão recebidos pelo rx, quando solicitado

        // o método .send() de tx retorna um Result<T, E>, onde retorna um Ok(T)
        // se rx (receptor) ainda não tenha sido destruido, caso contrário, retorna
        // um Err(_)

        // println!("{val}") Err: (move value) a chamada de send() toma posse de val
    });

    let received = rx.recv().unwrap();
    // rx.recv() bloqueia a thread atual até que receba um valor de tx.
    // Ele retorna um Result<T, E>: Ok(T) se o valor for recebido,
    // e Err(_) se o transmissor foi fechado antes de enviar algo.

    // try_recv verifica imediatamente se há uma mensagem disponível no canal.
    // Retorna Ok(T) se houver um valor, ou Err(_) se não houver nada ou se o canal tiver sido fechado.
    // Útil quando a thread não pode ficar bloqueada esperando dados e precisa continuar executando.
    println!("{received}");
}
