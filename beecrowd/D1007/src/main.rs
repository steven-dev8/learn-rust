use std::io;

fn main() {
    let (mut v1, mut v2, mut v3, mut v4) = (String::new(),
                                            String::new(),
                                            String::new(),
                                            String::new());
    io::stdin()
        .read_line(&mut v1)
        .expect("");

    io::stdin()
        .read_line(&mut v2)
        .expect("");

    io::stdin()
        .read_line(&mut v3)
        .expect("");

    io::stdin()
        .read_line(&mut v4)
        .expect("");

    let a_b = v1.trim().parse::<i64>().unwrap() * v2.trim().parse::<i64>().unwrap();
    let c_d = v3.trim().parse::<i64>().unwrap() * v4.trim().parse::<i64>().unwrap();

    println!("DIFERENCA = {}", a_b - c_d);
}
