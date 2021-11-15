fn main() {
    let s = String::from("hello");
    let (s2, len) = calculate_length(s);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    return (s, length);
}