

// you can have multiple immutable vaules but one mutable value and no mixing allowed

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}",r1,r2);

    let r3 = &mut s; // it's a mutable refernce 

r3.push_str(", world");
println!("{}",r3);
}



