use std::time::Duration;
use trpl::Either;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Hello Moto"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(mess) => println!("Succeeded with {mess}"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time), 
    }
}
