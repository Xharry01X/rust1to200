

// you can have multiple immutable vaules but one mutable value and no mixing allowed

fn main() {
    let mut x = 5;

    let y = &x;

    println!("{} value of x and value of Y {}",x,y);

    let z = &mut x;
    *z += 1;
    println!("Value of z is {}",z);
}



