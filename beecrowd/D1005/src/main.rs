use std::io;


fn main() {
    let mut nota_a = String::new();
    let mut nota_b = String::new();

    io::stdin()
        .read_line(&mut nota_a)
        .expect("Error");

    io::stdin()
        .read_line(&mut nota_b)
        .expect("Error");

    let nota_a : f64 = nota_a.trim().parse().unwrap();
    let nota_b : f64 = nota_b.trim().parse().unwrap();    

    let p1 = 3.5;
    let p2 = 7.5;

    let media = (nota_a * p1 + nota_b * p2) / (p1 + p2);
    println!("MEDIA = {:.5}", media);
}
