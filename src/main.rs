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

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    print!("첫 번째 숫자: ");
}
