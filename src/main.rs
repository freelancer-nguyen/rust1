use std::io;

fn main() {
    loop {
        println!("Please enter sequence number");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u16 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number");
                continue;
            }
        };
        let result = fib(n);
        println!("{n}th Fibonacci number is {result}");
    }
}

fn fib(n: u16) -> u128 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
