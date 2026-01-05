use std::io;
use std::collections::HashMap;

const GENERIC_ERROR: &str = "Error";

fn main() {
    let mut employee_db: HashMap<String, Vec<String>> = HashMap::new();

    println!("Welcome to The Management System");

    loop {
        println!("1 - Register employee");
        println!("2 - Employees by department");
        println!("3 - Show all employees by department");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect(GENERIC_ERROR);
        
        match user_input.chars().next().unwrap() {
            '1' => register_employee(&mut employee_db),
            '2' => show_employees_by_department(&employee_db), //{
                //if let Some(employees) = show_employees_by_department(&employee_db) {
                //    println!("{employees:?}");
               // }
            '3' => show_all_employees(&employee_db),
            _ => { println!("Please choose a valid option!"); continue }
        }
    }
}

fn register_employee(employee_db: &mut HashMap<String, Vec<String>>) {
    let mut department_name = String::new();
    let mut employee_name = String::new();

    io::stdin()
        .read_line(&mut department_name)
        .expect(GENERIC_ERROR);

    io::stdin()
        .read_line(&mut employee_name)
        .expect(GENERIC_ERROR);

    let department = department_name.trim().to_string();
    let employee = employee_name.trim().to_string();

    employee_db.entry(department)
        .and_modify(|employees| employees.push(employee.clone()))
        .or_insert(vec![employee]);
}

fn show_all_employees(employee_db: &HashMap<String, Vec<String>>) {
    for (department, employees) in employee_db {
        println!("{department}:");

        for (index, employee) in employees.iter().enumerate() {
            println!(" {} - {employee}", index + 1);
        }
    }
}

fn show_employees_by_department(employee_db: &HashMap<String, Vec<String>>){
    let mut department_name = String::new();

    io::stdin()
        .read_line(&mut department_name)
        .expect(GENERIC_ERROR);

    let department = department_name.trim().to_string();
    let list_department = employee_db.get(&department);


    if let Some(list_employess) = list_department {
        let mut employee_sorted = list_employess.clone();
        employee_sorted.sort();
        println!("{employee_sorted:? }")
    }
}
