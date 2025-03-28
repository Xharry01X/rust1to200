// loops concept

fn main(){
  let mut i = 1;

  
   loop {
    println!("{}",i);
    i += 1;

    if i > 100 {
        break;
    }

   }
}