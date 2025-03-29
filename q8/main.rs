

fn main(){
  let x = 5;
  let add_x = |y| x + y;

  println!("Capturing value of x {}", add_x(3));
}