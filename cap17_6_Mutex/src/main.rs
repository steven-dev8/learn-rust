use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;

fn main() {
    let val = Mutex::new(5);
    // Mutex é um ponteiro inteligente que atua como um árbitro:
    // ele gerencia o acesso seguro a um dado do tipo T, impedindo condições de corrida (data races)
    // em contextos concorrentes.

    // API do Mutex
    {
        let mut result = val.lock().unwrap();
        // .lock() tenta adquirir o bloqueio (lock) do Mutex.
        // Se outro thread já estiver com o lock, ele espera até que esteja disponível.
        // A chamada retorna um Result<T, E>:
        // - Ok(T): se conseguir o acesso
        // - Err(E): se o dado tiver sido corrompido (poisoned), ou seja, se um panic ocorreu
        // enquanto uma thread detinha o lock
        *result = 6;
        // Uma chamada bem-sucedida de .lock() retorna um MutexGuard,
        // que implementa Deref e DerefMut. Isso permite acessar e modificar o dado protegido.
    };
    // Ao final do escopo, o bloqueio (lock) é liberado automaticamente,
    // permitindo que outras threads acessem o dado.
    // Isso ocorre porque MutexGuard implementa a trait Drop.

    println!("{:?}", val);
}
