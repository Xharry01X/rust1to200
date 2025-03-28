fn main (){

    let numbers = [-10,3,5,-33,0];

    for number in numbers {
        if number < 0 {
            println!("{} is negative",number);
        } else if number > 0 {
            println!("{} This is positive",number);
        }else {
            println!("{} number is zero",number);
        }
    }
}