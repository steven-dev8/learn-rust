use std::io;

fn main() {
    let (mut employee_num, mut hour, mut hour_money) = (String::new(),
                                                    String::new(),
                                                    String::new());
    
    io::stdin().read_line(&mut employee_num).unwrap();
    io::stdin().read_line(&mut hour).unwrap();
    io::stdin().read_line(&mut hour_money).unwrap();

    let (employee_num, hour, hour_money) = (employee_num.trim().parse::<i64>().unwrap(),
                                        hour.trim().parse::<f64>().unwrap(),
                                        hour_money.trim().parse::<f64>().unwrap());
    
    println!("NUMBER = {employee_num}");
    println!("SALARY = U$ {:.2}", hour * hour_money);
}