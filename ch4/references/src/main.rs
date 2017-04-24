fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn change(a_string: &mut String) {
    a_string.push_str(", world!");
}

fn calculate_length(a_string: &String) -> usize {
    a_string.len()
}
