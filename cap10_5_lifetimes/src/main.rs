fn main() {
    // Parte 1
    let r;

    // { 'x vive menos que 'r
    //     let x = 5;
    //     r = &x;
    // }

    let x = 5;
    r = &x;

    println!("r: {r}");

    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result1 = longest(s1.as_str(), s2);
    println!("{result1}");

    // Parte 2
    let string1 = String::from("long string is long");
    // let result3;

    {
        let string2 = String::from("xyz");
        let result2 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result2}");

        // result3 = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {result3}");
    // Isso não daria certo, pois o lifetime deve ser menor do que
    // o lifetime de string1 e string2 | string2 nessa parte do código não existe mais
}

// &i32 uma referência
// &'a i32 uma referência com um lifetime explicito
// &'a mut i32 uma referência mutável com um lifetime explicito
fn example<'b>(x: &'b str, y: &str) -> &'b str {
    // O lifetime da referência de retorno deve ser relacionado ao lifetime de 'x,
    // porque estamos retornando uma referência a 'x'.
    // Se você definisse example(x: &str, y: &str) -> &'b str, isso não compilaria,
    // pois o compilador não saberia de onde viria a referência que estamos retornando,
    // já que 'x' não teria um lifetime especificado para associar ao lifetime da referência de retorno.
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// <'a> simboliza que aplicarei uma notação de lifetime
// x: &'a str significa que estou passando uma referência imutável de tipo 'str' 
// que tem o lifetime 'a', ou seja, a referência de `x` viverá até que a referência de `x` morra
// mesma coisa se aplica ao y

// -> &'a str: Aqui, estou dizendo que a referência retornada (seja de x ou y)
// deve ter o mesmo lifetime 'a', ou seja, ela só será válida enquanto x ou y existirem.

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
