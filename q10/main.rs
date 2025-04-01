
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}

fn main(){
    let person1 = Person {
        name: String::from("Harry"),
        age: 30,
        is_active: true
    };

    println!("{} is {} years old",person1.name, person1.age);
}