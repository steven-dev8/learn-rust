pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { // impl NOME_TRAIT for NOME_TIPO
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) { // Recebe um tipo que implementa a trait Summary
    println!("Breaking News! {}", item.summarize());
}
// A forma acima é equivalente a de baixo 
pub fn notify_two<T: Summary>(item1: &T, item2: &T) {
    println!("Two Breaking News! 1º: {} 2º: {}", item1.summarize(), item2.summarize());
}

pub fn notify_three<T: Summary + Clone>(item: &T) {}
// Estou especificando para o generic T, que ele tenha as trait Summary e Display implementadas

pub fn some_function<T, U>(t: &T, u: &U)
where
    T: Summary + Clone,
    U: Summary,
{}
// Uma forma mais legivel para generics que recebem mais de um Trait, utiliza a cláusula where