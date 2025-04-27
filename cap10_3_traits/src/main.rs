use aggregator::{NewsArticle, Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article: {}", article.summarize());
}

fn returns_summarizable() -> impl Summary { 
    // impl Summary, estou dizendo que posso retornar UM, e apenas UM, tipo que implementa 
    // essa trait

    Tweet { // Aqui eu retorno Tweet, então só posso retornar Tweet na função
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }

    // Se eu adicionasse outro tipo para retorno, o programa não compilaria
}
