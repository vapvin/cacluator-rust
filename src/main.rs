use std::io::{stdin, stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("Failed to Flush");
    stdin().read_line(input)
        .expect("Failed to Read");
}

fn main() {
    println!("심플 계산기");
    println!("-------------------");
    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        println!("첫 번째 숫자: ");
        read(&mut num1);

        println!("두 번째 숫자: ");
        read(&mut num2);

        println!("Operator: ");
        read(&mut operator);

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let num3: f32 = num1.trim().parse().unwrap();
        let num4: f32 = num2.trim().parse().unwrap();
        let operator2: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator")
        };
        
        println!("the result of {} {} {} = {}", num1, operator, num2, result);
    }
}
