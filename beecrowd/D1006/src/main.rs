use std::io;

fn main() {
    let (mut v1, mut v2, mut v3) = (String::new(), String::new(), String::new());

    io::stdin()
        .read_line(&mut v1)
        .expect("");
    
    io::stdin()
        .read_line(&mut v2)
        .expect("");
    
    io::stdin()
        .read_line(&mut v3)
        .expect("");
        
    let (p1, p2, p3) = (2.0, 3.0, 5.0);
    
    let media = (v1.trim().parse::<f64>().unwrap() * p1 +
                 v2.trim().parse::<f64>().unwrap() * p2 +
                 v3.trim().parse::<f64>().unwrap() * p3) /
                (p1 + p2 + p3);

    println!("MEDIA = {:.1}", media);
}