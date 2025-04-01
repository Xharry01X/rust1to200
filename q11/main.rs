
struct Color(u8,u8,u8);

fn main(){
  let black  = Color(0,0,0);
  let white = Color(255,255,255);

  println!("Black: R{}, G{}, B{}", black.0, black.1,black.2);
  println!("White: R{}, G{}, B{}",white.0, white.1, white.2);


}