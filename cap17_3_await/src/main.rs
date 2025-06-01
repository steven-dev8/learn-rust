use std::time::Duration;

fn main() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..5 {
                println!("This is number {i} from first");
                trpl::sleep(Duration::from_millis(500)).await;
            };
        };


        for i in 1..10 {
            println!("This is number {i} from second");
            trpl::sleep(Duration::from_millis(500)).await;
        };

        fut1.await;
    })
}
