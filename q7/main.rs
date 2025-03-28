fn main(){
    let input = "Education";

    let vowels_count = count_vowels(input);
    println!("Numbers of vowels in '{}' : {}",input, vowels_count);


}

fn count_vowels(s: &str) -> usize {
    let vowels = ['a','e','i','o','u'];

    s.chars()
     .filter(|c| vowels.contains(&c.to_ascii_lowercase()))
     .count()
}