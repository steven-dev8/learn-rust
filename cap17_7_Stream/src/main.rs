use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|x| x * 2);
        let mut stream = trpl::stream_from_iter(iter);

        // Stream são iteradores assincronos, eles não bloqueiam a execução do programa

        while let Some(value) = stream.next().await {
            println!("The values of array is {value}");
        };

        let values = 1..101;
        let iter = values.map(|x| x * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|x| x % 3 == 0 || x % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value is {value}");
        };
    })
}
