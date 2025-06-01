use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));

    let x1 = Arc::clone(&x);
    let y1 = Arc::clone(&y);

    let t1 = thread::spawn(move || {
        let _lock_x = x1.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let _lock_y = y1.lock().unwrap();
    });

    let x2 = Arc::clone(&x);
    let y2 = Arc::clone(&y);

    let t2 = thread::spawn(move || {
        let _lock_y = y2.lock().unwrap();
        let _lock_x = x2.lock().unwrap();
    });
    // Exemplo de deadlock induzido
    // lock_x da t1 bloqueia X, lock_y da t2 bloqueia Y
    // logo após t1 tenta pedir acesso a Y, mas Y ta bloquedo na thread em t2
    // t2 também tenta acessar X, porém X ta bloqueado na thread em t1

    // Conclusão
    // Esse problema é o que chamamos de DEADLOCK
    // quando um dado X está bloqueado por uma thread e precisa do dado Y,
    // porém o dado Y está bloqueado em outra thread que também precisa do dado X

    t1.join().unwrap();
    t2.join().unwrap();

    // Traits Send e Sync

    // Send: Permite que o *ownership* do valor seja movido de uma thread para outra
    // Ex: thread::spawn(move || { /* usa o valor */ })

    // Sync: Permite que múltiplas threads acessem *referências imutáveis* (&T) ao mesmo tempo
    // Ex: Arc<T> e Mutex<T> implementam Sync se T também for Sync
}
