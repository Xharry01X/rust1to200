

fn main(){

    let my_tuple: (i32, f64, &str) = (42,3.14, "hello");


    println!("First element: {}", my_tuple.0);
    println!("Second element: {}",my_tuple.1);
    println!("Third element: {}",my_tuple.2);

    let (a,b,c) = my_tuple;
    println!("Desturcted: {} {} {}",a,b,c);                                                                                                                                                           
}