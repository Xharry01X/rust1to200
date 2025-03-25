fn main() {
    let radius: f64 = 5.0;  

    let pi = std::f64::consts::PI;  
    
    let area = pi * radius.powi(2);  
    
    println!("Area of the circle with radius {} is: {:.2}", radius, area);
    
}