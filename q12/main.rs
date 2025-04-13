

fn main() {
    let a = 1;
    let b = 3;

    let result = print_sum(a, b);
    println!("The sum is: {}", result);
}

fn print_sum(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    sum = a + b;
    sum
}

