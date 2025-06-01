use std::time::Duration;
use std::pin::Pin;
use std::pin::pin;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("Oi");
        tx.send(val).unwrap();
    
    
        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });
    // Todo bloco ou função marcado como "async" cria e retorna uma Future
    // Futures são lazy: só executam quando aguardadas com `.await` ou "polladas" por um executor
    // `.await` suspende a execução até a Future ser concluída

    // rx.recv() retorna uma Future
    // Delegamos o gerenciamento das Futures para o trpl::run que é um executor

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_clone = tx.clone();

        let tx_fut = pin!(async move { 
        // `async move` transfere a posse (ownership) das variáveis capturadas para dentro do bloco assíncrono
            let vec = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for vals in vec {
                tx.send(vals).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            };
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                // rx.recv() só é fechado quando recebe None, só há duas condições pra isso acontecer
                // chamando manualmente o rx.close ou o transmissor (tx) for destruido
                println!("Received: {val}");
            };
        });

        let tx_clone_fut = pin!(async move {
            let vec = vec![
                String::from("Estou"),
                String::from("usando"),
                String::from("o (tx)"),
                String::from("clone"),
            ];

            for val in vec {
                tx_clone.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            };
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>>
            = vec![tx_fut, rx_fut, tx_clone_fut];

        trpl::join_all(futures).await;
            // Aguarda as Futures serem executadas até o fim antes de encerrar o bloco
    })
}
