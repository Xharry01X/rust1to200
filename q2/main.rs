

fn main(){

    let numbers: [i32; 5] = [10,20,30,40,50];

    print_length(&numbers);

    print_content(&numbers);

}

fn print_length(arr: &[i32]){
      println!("Array size is: {}",arr.len());
}

fn print_content(arr: &[i32]){
    println!("Arrays contents is: {:?}",arr);
}