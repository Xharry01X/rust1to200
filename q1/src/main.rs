
fn add(a: i32, b: i32) -> i32 {
    a + b
}


fn sum(a: i32, b: i32) -> i32 {
    add(a, b) 
}

fn main() {
    let num1 = 10;
    let num2 = 20;
    let result = sum(num1, num2);  
    println!("The sum of {} and {} is: {}", num1, num2, result);
}