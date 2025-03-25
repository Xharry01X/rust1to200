


fn main() {
    let x =5;
    println!("{}",x);

    let x = x + 1; //shadowing the original value of x
    println!("Shadow x: {}", x);
}
