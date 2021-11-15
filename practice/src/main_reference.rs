// reference
fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    change(&mut s);
    println!("The changed s was '{}'", s);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    return length;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}